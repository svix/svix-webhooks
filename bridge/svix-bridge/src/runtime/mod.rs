use std::num::NonZeroUsize;

use anyhow::Result;
use deadpool::unmanaged::Pool;
use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_runtime::deno_core::{
    serde_v8,
    v8::{self},
    JsRuntime,
};
use svix_bridge_types::{JsObject, TransformerInput, TransformerOutput};
use tokio::sync::oneshot;

struct Executor {
    tx: std::sync::mpsc::Sender<Job>,
    _handle: std::thread::JoinHandle<()>,
}

impl Default for Executor {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Job>();
        let _handle = std::thread::spawn(move || {
            let mut runtime = JsRuntime::new(Default::default());
            for Job { input, script, cb } in rx {
                let ret = run_script_inner(&mut runtime, input, script);
                if cb.send(ret).is_err() {
                    tracing::error!("failed to send script output to caller");
                }
            }
        });
        Self { tx, _handle }
    }
}

type Callback = oneshot::Sender<Result<TransformerOutput>>;

struct Job {
    input: TransformerInput,
    script: String,
    cb: Callback,
}

impl Executor {
    async fn execute(
        &mut self,
        input: TransformerInput,
        script: String,
    ) -> Result<TransformerOutput> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job {
            input,
            script,
            cb: tx,
        })?;
        rx.await?
    }
}

#[derive(Clone)]
pub struct JsPooler {
    executors: Pool<Executor>,
}

impl JsPooler {
    pub fn new(pool_size: NonZeroUsize) -> Self {
        let pool_size = pool_size.get();
        let mut items = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(Executor::default());
        }
        Self {
            executors: Pool::from(items),
        }
    }

    pub async fn run_script(
        &self,
        input: TransformerInput,
        script: String,
    ) -> Result<TransformerOutput> {
        let pool = self.executors.clone();
        let mut executor = pool.get().await;

        executor
            .as_mut()
            .map_err(|e| anyhow::anyhow!("{e:?}"))?
            .execute(input, script)
            .await
    }
}

/// Checks that the input parses as valid JavaScript, giving the parser's error back on failure.
pub fn validate_script(src: &str) -> Result<()> {
    Ok(deno_ast::parse_script(ParseParams {
        specifier: "file:///x.js".to_string(),
        text_info: SourceTextInfo::new(src.into()),
        media_type: MediaType::JavaScript,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })
    .map(|_| ())?)
}

fn run_script_inner(
    runtime: &mut JsRuntime,
    input: TransformerInput,
    script: String,
) -> Result<TransformerOutput> {
    let input = serde_json::to_string(&input)?;
    let res = runtime.execute_script(
        "<anon>",
        format!(
            // Wrap the user script, and invocation of `handler`, in a self-calling closure.
            // The hope is we'll prevent the globals space from being polluted call after call.
            r#"
    (function () {{
        {script}
        return handler({});
    }})()
    "#,
            input
        )
        .into(),
    );
    match res {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            match serde_v8::from_v8::<JsObject>(scope, local) {
                Ok(v) => Ok(TransformerOutput::Object(v)),
                Err(serde_v8::Error::ExpectedObject(msg)) => {
                    tracing::error!("{msg}");
                    Ok(TransformerOutput::Invalid)
                }
                Err(e) => Err(e)?,
            }
        }
        Err(err) => Err(anyhow::format_err!("Evaling error: {:?}", err)),
    }
}

#[cfg(test)]
mod tests;
