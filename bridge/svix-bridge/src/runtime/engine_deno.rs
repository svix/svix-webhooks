use anyhow::Result;
use deno_core::{
    JsRuntime, extension, serde_v8,
    v8::{self},
};
use svix_bridge_types::{JsObject, TransformerInput, TransformerOutput};

pub fn build_runtime() -> JsRuntime {
    JsRuntime::new(deno_core::RuntimeOptions {
        extensions: vec![svix_disable_ops::init()],
        ..Default::default()
    })
}

extension!(
    svix_disable_ops,
    middleware = |op: deno_core::OpDecl| match op.name {
        "op_panic" => op.disable(),
        _ => op,
    },
    docs = "Disable dangerous builtin ops"
);

pub(super) fn run_script_inner(
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
        return handler({input});
    }})()
    "#,
        ),
    );
    match res {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            match serde_v8::from_v8::<JsObject>(scope, local) {
                Ok(v) => Ok(TransformerOutput::Object(v)),
                Err(e @ serde_v8::Error::ExpectedObject(_)) => {
                    tracing::error!("{e}");
                    Ok(TransformerOutput::Invalid)
                }
                Err(e) => {
                    tracing::error!("{e}");
                    Err(e)?
                }
            }
        }
        Err(err) => Err(anyhow::format_err!("Evaling error: {err:?}")),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use serde_json::json;
    use svix_bridge_types::{TransformerInput, TransformerOutput};

    use super::{build_runtime, run_script_inner};

    // Really just trying to figure out if the deno runtime is working the way I hope.
    #[test]
    fn test_happy_fn() {
        let src = r#"
        function handler(input) {
            return { "x": 123, ...input };
        }
        "#
        .to_string();
        let mut rt = build_runtime();
        let res = run_script_inner(&mut rt, json!({ "y": 456 }).into(), src).unwrap();
        let TransformerOutput::Object(v) = res else {
            panic!("got unexpected return value");
        };
        assert_eq!(v["x"].as_i64(), Some(123));
        assert_eq!(v["y"].as_i64(), Some(456));
    }

    #[test]
    fn test_invalid_output_bool() {
        let src = r#"
        function handler(input) {
            return false;
        }
        "#
        .to_string();

        let mut rt = build_runtime();
        let res = run_script_inner(&mut rt, json!({}).into(), src).unwrap();
        assert_matches!(res, TransformerOutput::Invalid);
    }

    #[test]
    // FIXME: serde_v8 decodes arrays with keys like "0", "1"... in this situation, failing the test.
    #[ignore]
    fn test_invalid_output_array() {
        let src = r#"
        function handler(input) {
            return [1, 2];
        }
        "#
        .to_string();
        let mut rt = build_runtime();
        let res = run_script_inner(&mut rt, json!({}).into(), src).unwrap();
        assert_matches!(res, TransformerOutput::Invalid);
    }

    /// Receives a string input, parses as JSON in js, then returns the result back to rust.
    #[test]
    fn test_string_input() {
        let src = r#"
        function handler(input) {
            return JSON.parse(input);
        }
        "#
        .to_string();
        let mut rt = build_runtime();
        let res = run_script_inner(
            &mut rt,
            TransformerInput::String(String::from(r#"{"x": 123}"#)),
            src,
        )
        .unwrap();
        let v = res.unwrap();
        assert_eq!(v["x"].as_i64(), Some(123));
    }

    /// Take the string input and just add it to a field in the returned object.
    /// The string should make it through, back to rust, as-is.
    #[test]
    fn test_string_input2() {
        let src = r#"
        function handler(input) {
            return { "payload": input };
        }
        "#
        .to_string();
        let mut rt = build_runtime();
        let res = run_script_inner(
            &mut rt,
            TransformerInput::String(String::from("Hello World")),
            src,
        )
        .unwrap();
        let v = res.unwrap();
        assert_eq!(v["payload"].as_str(), Some("Hello World"));
    }

    #[tokio::test]
    async fn test_panic() {
        let test_script = r#"
            function handler(input) {
                Deno.core.ops.op_panic("Good bye");
                return input;
            }
        "#;

        let mut rt = build_runtime();
        let err = run_script_inner(
            &mut rt,
            TransformerInput::String("".to_owned()),
            test_script.to_owned(),
        )
        .unwrap_err()
        .to_string();
        if !err.contains("op is disabled") {
            panic!("Wrong error {err}")
        }
    }
}
