use svix::api::ListResponseApplicationOut;

#[tokio::test]
async fn test_list_response_xxx_out() {
    // first test with iterator and prevIterator
    let json_str =
        r#"{"data":[],"done":true,"iterator":"iterator-str","prevIterator":"prevIterator-str"}"#;
    let loaded_json: ListResponseApplicationOut = serde_json::from_str(json_str).unwrap();

    let expected_model = ListResponseApplicationOut {
        data: vec![],
        done: true,
        iterator: Some("iterator-str".to_string()),
        prev_iterator: Some("prevIterator-str".to_string()),
    };

    assert_eq!(expected_model, loaded_json);

    // without iterator and prevIterator
    let json_str = r#"{"data":[],"done":true,"iterator":null,"prevIterator":null}"#;
    let loaded_json: ListResponseApplicationOut = serde_json::from_str(json_str).unwrap();

    let expected_model = ListResponseApplicationOut {
        data: vec![],
        done: true,
        ..Default::default()
    };

    assert_eq!(expected_model, loaded_json);
}
