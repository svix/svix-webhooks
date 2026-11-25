use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use rquickjs::{
    CatchResultExt, Class, Ctx, Function, JsLifetime, Promise, Value,
    class::Trace,
    context::EvalOptions,
    function::{Opt, Rest},
    prelude::Func,
};
use svix_bridge_types::{TransformerInput, TransformerOutput};
use thiserror::Error;
use tokio::sync::Notify;
use tracing::{Instrument as _, field::Empty};

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("internal error setting up the JavaScript interpreter: {0}")]
    InternalError(rquickjs::Error),
    #[error("unable to serialize input: {0}")]
    InvalidInput(rquickjs_serde::Error),
    #[error("exception raised by JavaScript code: {message:?} {}", stack.as_deref().unwrap_or(""))]
    ExecutionException {
        message: String,
        stack: Option<String>,
    },
    #[error("Evaluating the top-level module failed to return a valid promise")]
    InvalidPromise,
    #[error("maximum processing time exceeded")]
    ProcessTimeout,
    #[error("maximum RAM use exceeded")]
    OutOfMemory,
}

#[derive(Clone, Trace, JsLifetime)]
#[rquickjs::class(frozen)]
struct Console {}

#[rquickjs::methods]
impl Console {
    fn debug(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("debug called from script");
        Ok(())
    }

    fn log(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("log called from script");
        Ok(())
    }

    fn warn(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("warn called from script");
        Ok(())
    }

    fn error(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("error called from script");
        Ok(())
    }
}

#[derive(Trace, JsLifetime)]
#[rquickjs::class]
struct Timeout {
    #[qjs(skip_trace)]
    abort: Arc<Notify>,
}

fn clear_timeout<'js>(_ctx: Ctx<'js>, timeout: Class<'js, Timeout>) -> rquickjs::Result<()> {
    timeout.borrow().abort.notify_one();
    Ok(())
}

fn set_timeout<'js>(
    ctx: Ctx<'js>,
    cb: Function<'js>,
    millis: Opt<u64>,
) -> rquickjs::Result<Class<'js, Timeout>> {
    // JS always sleeps for at least 4ms
    let duration = Duration::from_millis(millis.0.unwrap_or(0).max(4));
    let abort = Arc::new(Notify::new());
    let abort_bg = Arc::clone(&abort);
    ctx.spawn(async move {
        let aborted = tokio::select! {
            _ = tokio::time::sleep(duration) => false,
            _ = abort_bg.notified() => true
        };
        if aborted {
            tracing::trace!("ignoring callback because we were aborted via clearTimeout");
        } else if let Err(err) = cb.call::<(), ()>(()) {
            tracing::error!("Failed to call callback: {err}");
        }
    });
    Class::instance(ctx, Timeout { abort })
}

fn handle_caught_error(e: rquickjs::CaughtError<'_>) -> ScriptError {
    match e {
        rquickjs::CaughtError::Exception(exc) => match exc.message().as_deref() {
            Some("interrupted") => ScriptError::ProcessTimeout,
            Some("out of memory") => ScriptError::OutOfMemory,
            _ => ScriptError::ExecutionException {
                message: exc
                    .message()
                    .unwrap_or_else(|| "<unknown error>".to_string()),
                stack: exc.stack(),
            },
        },
        rquickjs::CaughtError::Error(err) => {
            let stringified = format!("{err:?}");
            ScriptError::ExecutionException {
                message: stringified,
                stack: None,
            }
        }
        rquickjs::CaughtError::Value(v) => {
            // OOM is represented by raising `null` as an exception, instead of some dedicated
            // singleton
            if v.is_null() {
                ScriptError::OutOfMemory
            } else {
                let stringified = format!("{v:?}");
                ScriptError::ExecutionException {
                    message: stringified,
                    stack: None,
                }
            }
        }
    }
}

#[tracing::instrument(skip_all, level="debug", fields(error=Empty, runtime_us=Empty, max_memory_bytes=Empty))]
pub async fn run_script(
    input: TransformerInput,
    script: String,
    max_duration: Duration,
) -> Result<TransformerOutput> {
    let start = Instant::now();
    let result = run_script_inner(input, script, max_duration)
        .instrument(tracing::Span::current())
        .await;
    if let Err(e) = &result {
        tracing::warn!(
            err = ?e,
            runtime_us = start.elapsed().as_micros(),
            "error executing transformation"
        );
    }
    result.map_err(Into::into)
}

// TODO: move to cfg
const MAX_RAM_BYTES: usize = 26_214_400;

async fn init_context(
    runtime: &rquickjs::AsyncRuntime,
) -> Result<rquickjs::AsyncContext, ScriptError> {
    let context = rquickjs::AsyncContext::full(runtime)
        .await
        .map_err(ScriptError::InternalError)?;
    context
        .with(move |ctx| {
            let globals = ctx.globals();
            globals
                .set("console", Console {})
                .map_err(ScriptError::InternalError)?;
            globals
                .set("clearTimeout", Func::from(clear_timeout))
                .map_err(ScriptError::InternalError)?;
            globals
                .set("setTimeout", Func::from(set_timeout))
                .map_err(ScriptError::InternalError)?;
            Ok(())
        })
        .await?;
    Ok(context)
}

fn options() -> EvalOptions {
    assign::assign!(EvalOptions::default(), {
        global: true,
        strict: false,
        promise: true,
        backtrace_barrier: true,
        filename: Some("transformation_input.js".to_owned())
    })
}

async fn exec_script<'a>(
    ctx: Ctx<'a>,
    script: String,
    deadline: Instant,
) -> Result<rquickjs::Value<'a>, ScriptError> {
    let future = ctx
        .eval_with_options::<Promise<'_>, String>(script, options())
        .catch(&ctx)
        .map_err(handle_caught_error)?;
    // A script can await a promise that never settles, and while it does so no JavaScript is
    // running for the interrupt handler to interrupt, so the same deadline is enforced here
    // too.
    tokio::time::timeout_at(
        tokio::time::Instant::from_std(deadline),
        future.into_future::<rquickjs::Value<'_>>(),
    )
    .await
    .map_err(|_elapsed| ScriptError::ProcessTimeout)?
    .catch(&ctx)
    .map_err(handle_caught_error)
}

async fn run_script_inner(
    input: TransformerInput,
    mut script: String,
    max_duration: Duration,
) -> Result<TransformerOutput, ScriptError> {
    let runtime = rquickjs::AsyncRuntime::new().map_err(ScriptError::InternalError)?;
    runtime.set_memory_limit(MAX_RAM_BYTES).await;

    let context = init_context(&runtime).await?;

    context
        .with(move |ctx| {
            let globals = ctx.globals();
            let serialized =
                rquickjs_serde::to_value(ctx, &input).map_err(ScriptError::InvalidInput)?;
            globals
                .set("script_input", serialized)
                .map_err(ScriptError::InternalError)?;
            Ok(())
        })
        .instrument(tracing::debug_span!("rquickjs_serialize"))
        .await?;

    let start = Instant::now();
    runtime
        .set_interrupt_handler(Some(Box::new(move || start.elapsed() > max_duration)))
        .await;

    let value = context
        .async_with(async |ctx| {
            // The script is evaluated as an async function (see `promise` above), so awaiting here
            // lets `handler` be async as well; awaiting a plain value is a no-op.
            script.push_str(";\n await handler(script_input)");

            let value = exec_script(ctx, script, start + max_duration)
                .instrument(tracing::debug_span!("rquickjs_execute"))
                .await?;

            // https://github.com/DelSkayn/rquickjs/issues/360
            let object = value.as_object().ok_or(ScriptError::InvalidPromise)?;
            let value: Value<'_> = object
                .get("value")
                .map_err(|_| ScriptError::InvalidPromise)?;

            tracing::debug_span!("rquickjs_deserialize").in_scope(|| {
                match rquickjs_serde::from_value(value) {
                    Ok(value) => Ok(TransformerOutput::Object(value)),
                    Err(e) => {
                        tracing::warn!(?e, "transformer output was not a JSON object");
                        Ok(TransformerOutput::Invalid)
                    }
                }
            })
        })
        .await?;

    let span = tracing::Span::current();
    let duration = start.elapsed();
    span.record("duration_us", duration.as_micros());

    let usage = runtime.memory_usage().await;
    span.record("max_memory_bytes", usage.memory_used_size);

    Ok(value)
}

/// Checks that the input parses as valid JavaScript, giving the parser's error back on failure.
pub async fn validate_script(src: &str) -> Result<()> {
    let runtime = rquickjs::AsyncRuntime::new().map_err(ScriptError::InternalError)?;
    runtime.set_memory_limit(MAX_RAM_BYTES).await;

    let max_duration = Duration::from_millis(20);

    let context = init_context(&runtime).await?;

    let start = Instant::now();
    runtime
        .set_interrupt_handler(Some(Box::new(move || start.elapsed() > max_duration)))
        .await;

    context
        .async_with(async |ctx| {
            let mut script = "(function wrapper() { ".to_owned();
            script.push_str(src);
            script.push_str("})");

            exec_script(ctx, script, start + max_duration)
                .instrument(tracing::debug_span!("rquickjs_validate"))
                .await?;

            Ok(())
        })
        .await
}

#[cfg(test)]
mod tests {
    use std::{
        assert_matches,
        time::{Duration, Instant},
    };

    use serde_json::json;
    use svix_bridge_types::{TransformerInput, TransformerOutput};

    use super::{ScriptError, run_script_inner, validate_script};

    const MAX_RUNTIME: Duration = Duration::from_secs(5);

    // Really just trying to figure out if the deno runtime is working the way I hope.
    #[tokio::test]
    async fn test_happy_fn() {
        let src = r#"
        function handler(input) {
            return { "x": 123, ...input };
        }
        "#
        .to_string();
        let res = run_script_inner(json!({ "y": 456 }).into(), src, MAX_RUNTIME)
            .await
            .unwrap();
        match res {
            TransformerOutput::Object(v) => {
                assert_eq!(v["x"].as_i64(), Some(123));
                assert_eq!(v["y"].as_i64(), Some(456));
            }
            TransformerOutput::Invalid => panic!("got unexpected return value"),
        }
    }

    #[tokio::test]
    async fn test_invalid_output_bool() {
        let src = r#"
        function handler(input) {
            return false;
        }
        "#
        .to_string();

        let res = run_script_inner(json!({}).into(), src, MAX_RUNTIME)
            .await
            .unwrap();
        assert_matches!(res, TransformerOutput::Invalid);
    }

    #[tokio::test]
    async fn test_invalid_output_string() {
        let src = r#"
        function handler(input) {
            return "hello, dolly";
        }
        "#
        .to_string();

        let res = run_script_inner(json!({}).into(), src, MAX_RUNTIME)
            .await
            .unwrap();
        assert_matches!(res, TransformerOutput::Invalid);
    }

    #[tokio::test]
    async fn test_invalid_output_array() {
        let src = r#"
        function handler(input) {
            return [1, 2];
        }
        "#
        .to_string();
        let res = run_script_inner(json!({}).into(), src, MAX_RUNTIME)
            .await
            .unwrap();
        assert_matches!(res, TransformerOutput::Invalid);
    }

    /// Receives a string input, parses as JSON in js, then returns the result back to rust.
    #[tokio::test]
    async fn test_string_input() {
        let src = r#"
        function handler(input) {
            return JSON.parse(input);
        }
        "#
        .to_string();
        let res = run_script_inner(
            TransformerInput::String(String::from(r#"{"x": 123}"#)),
            src,
            MAX_RUNTIME,
        )
        .await
        .unwrap();
        let v = res.unwrap();
        assert_eq!(v["x"].as_i64(), Some(123));
    }

    /// Take the string input and just add it to a field in the returned object.
    /// The string should make it through, back to rust, as-is.
    #[tokio::test]
    async fn test_string_input2() {
        let src = r#"
        function handler(input) {
            return { "payload": input };
        }
        "#
        .to_string();
        let res = run_script_inner(
            TransformerInput::String(String::from("Hello World")),
            src,
            MAX_RUNTIME,
        )
        .await
        .unwrap();
        let v = res.unwrap();
        assert_eq!(v["payload"].as_str(), Some("Hello World"));
    }

    #[tokio::test]
    async fn test_max_runtime_is_enforced() {
        let src = r#"
        async function handler(input) {
            const sleep = m => new Promise(r => setTimeout(r, m));
            await sleep(2000);
            return "ok";
        }
        "#
        .to_owned();
        let start = Instant::now();
        let res = run_script_inner(
            TransformerInput::String(String::from("Hello World")),
            src,
            Duration::from_millis(5),
        )
        .await;
        let dur = start.elapsed();
        std::assert_matches!(res, Err(ScriptError::ProcessTimeout));
        assert!(dur >= Duration::from_millis(4));
        assert!(dur < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_unsettled_promise_gets_killed() -> anyhow::Result<()> {
        let start = Instant::now();
        let response = run_script_inner(
            TransformerInput::String("".to_owned()),
            r#"function handler(input) { return new Promise(() => {}); }"#.to_owned(),
            Duration::from_millis(10),
        )
        .await;
        let dur = start.elapsed();
        let Err(ScriptError::ProcessTimeout) = response else {
            anyhow::bail!("expected a timeout; got {response:?}");
        };
        assert!(dur >= Duration::from_millis(9));
        assert!(dur <= Duration::from_millis(100));
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_script_bad_syntax_is_err() {
        assert_matches!(validate_script("let 123 = ';").await, Err(_));
        assert_matches!(validate_script("function handler() {").await, Err(_));
    }

    #[tokio::test]
    async fn test_validate_script_empty_handler_is_ok() {
        assert_matches!(validate_script("function handler() { }").await, Ok(_));
    }

    #[tokio::test]
    async fn test_validate_script_arrow_fn_is_ok() {
        assert_matches!(
            validate_script("const handler = () => ({ a: 123 })").await,
            Ok(_)
        );
    }

    /// Technically, this should be legal though the utility is questionable.
    #[tokio::test]
    async fn test_validate_script_empty_is_ok() {
        assert_matches!(validate_script("").await, Ok(_));
        assert_matches!(validate_script("    ").await, Ok(_));
    }

    #[tokio::test]
    async fn test_validate_script_does_not_detect_nonexistent_function_calls() {
        assert_matches!(
            validate_script("function handler() { this_does_not_exist() }").await,
            Ok(_)
        );
    }

    #[tokio::test]
    async fn test_validate_script_does_not_actually_run_the_script_mostly() {
        assert_matches!(
            validate_script("function handler() { while(1) { console.log('1'); } }").await,
            Ok(_)
        );
        assert_matches!(
            validate_script("while(1) { console.log('1'); }").await,
            Ok(_)
        );
    }

    #[tokio::test]
    async fn test_validate_script_catches_clever_wrapper_escapes() {
        // we prevent running the script by wrapping it in an anonymous function; a very clever
        // malicious config writer could escape this by closing the curly braces. we still put
        // a total timeout on execution, so the best they can do is annoy themselves
        assert_matches!(
            validate_script("});\nwhile(1) { console.log('1') } ; (function f() {").await,
            Err(e) if e.to_string() == "maximum processing time exceeded"
        );
    }
}
