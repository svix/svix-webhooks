use std::str::FromStr;

use anyhow::Result;
use deno_ast::{MediaType, ParseParams};
use deno_core::{
    JsRuntime, extension, serde_v8, url,
    v8::{self},
};
use svix_bridge_types::{JsObject, TransformerInput, TransformerOutput};

/// Checks that the input parses as valid JavaScript, giving the parser's error back on failure.
pub(super) fn validate_script(src: &str) -> Result<()> {
    Ok(deno_ast::parse_script(ParseParams {
        specifier: url::Url::from_str("file:///x.js").expect("static string"),
        text: src.into(),
        media_type: MediaType::JavaScript,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })
    .map(|_| ())?)
}

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
    use serde_json::json;
    use svix_bridge_types::{TransformerInput, TransformerOutput};

    use super::{build_runtime, run_script_inner, validate_script};

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
        match res {
            TransformerOutput::Object(v) => {
                assert_eq!(v["x"].as_i64(), Some(123));
                assert_eq!(v["y"].as_i64(), Some(456));
            }
            TransformerOutput::Invalid => panic!("got unexpected return value"),
        }
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
        match res {
            TransformerOutput::Invalid => (),
            TransformerOutput::Object(_) => panic!("got unexpected return value"),
        }
    }

    #[test]
    // FIXME: serde decodes arrays with keys like "0", "1"... in this situation, failing the test.
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
        match res {
            TransformerOutput::Invalid => (),
            TransformerOutput::Object(_) => {
                panic!("got unexpected return value");
            }
        }
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
        match res {
            TransformerOutput::Object(v) => {
                assert_eq!(v["x"].as_i64(), Some(123));
            }
            TransformerOutput::Invalid => (),
        }
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
        match res {
            TransformerOutput::Object(v) => {
                assert_eq!(v["payload"].as_str(), Some("Hello World"));
            }
            TransformerOutput::Invalid => (),
        }
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

    #[test]
    fn test_validate_script_bad_syntax_is_err() {
        assert!(validate_script("let 123 = ';").is_err());
    }

    #[test]
    fn test_validate_script_empty_handler_is_ok() {
        assert!(validate_script("function handler() { }").is_ok());
    }

    #[test]
    fn test_validate_script_arrow_fn_is_ok() {
        assert!(validate_script("const handler = () => ({ a: 123 })").is_ok());
    }

    /// Technically, this should be legal though the utility is questionable.
    #[test]
    fn test_validate_script_empty_is_ok() {
        assert!(validate_script("").is_ok());
        assert!(validate_script("    ").is_ok());
    }
}
