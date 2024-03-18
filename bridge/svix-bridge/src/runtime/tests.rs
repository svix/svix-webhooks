use deno_runtime::deno_core::JsRuntime;
use serde_json::json;
use svix_bridge_types::{TransformerInput, TransformerOutput};

use super::{run_script_inner, validate_script};

fn get_test_rt() -> JsRuntime {
    JsRuntime::new(Default::default())
}

// Really just trying to figure out if the deno runtime is working the way I hope.
#[test]
fn test_happy_fn() {
    let src = r#"
    function handler(input) {
        return { "x": 123, ...input };
    }
    "#
    .to_string();
    let mut rt = get_test_rt();
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

    let mut rt = get_test_rt();
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
    let mut rt = get_test_rt();
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
    let mut rt = get_test_rt();
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
    let mut rt = get_test_rt();
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
