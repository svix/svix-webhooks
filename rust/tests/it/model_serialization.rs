use std::{collections::BTreeMap, fmt::Debug};

use serde::de::DeserializeOwned;
use serde_json::json;
use svix::models::{
    CronConfig, IngestSourceIn, IngestSourceInConfig, IngestSourceOut, IngestSourceOutConfig,
    ListResponseApplicationOut, SegmentConfig, SegmentConfigOut, SvixConfig, SvixConfigOut,
};

#[test]
fn test_list_response_xxx_out() {
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
        iterator: None,
        prev_iterator: None,
    };

    assert_eq!(expected_model, loaded_json);
}

#[test]
fn test_ingest_source_in() {
    assert_eq!(
        json!(IngestSourceIn {
            name: "foo".to_owned(),
            uid: None,
            config: IngestSourceInConfig::GenericWebhook,
            metadata: Some(BTreeMap::new())
        }),
        json!({
            "name": "foo",
            "type": "generic-webhook",
            "metadata": {}
        }),
    );

    assert_eq!(
        json!(IngestSourceIn {
            name: "foo".to_owned(),
            uid: None,
            config: IngestSourceInConfig::Svix(SvixConfig {
                secret: "xxx".to_owned()
            }),
            metadata: Some(BTreeMap::new())
        }),
        json!({
            "name": "foo",
            "type": "svix",
            "config": { "secret": "xxx" },
            "metadata": {}
        }),
    );

    assert_eq!(
        json!(IngestSourceIn {
            name: "foo".to_owned(),
            uid: None,
            config: IngestSourceInConfig::Segment(SegmentConfig { secret: None }),
            metadata: Some(BTreeMap::new())
        }),
        json!({
            "name": "foo",
            "type": "segment",
            "config": {},
            "metadata": {}
        }),
    );

    assert_eq!(
        json!(IngestSourceIn {
            name: "foo".to_owned(),
            uid: None,
            config: IngestSourceInConfig::Cron(CronConfig {
                content_type: None,
                payload: "💣".to_owned(),
                schedule: "* * * * *".to_owned(),
            }),
            metadata: Some(BTreeMap::new())
        }),
        json!({
            "name": "foo",
            "type": "cron",
            "config": {
                "payload": "💣",
                "schedule": "* * * * *",
            },
            "metadata": {}
        }),
    );
}

#[test]
fn test_ingest_source_out() {
    assert_deserializes_to(
        json!({
            "id": "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr",
            "createdAt": "2006-01-02T15:04:05Z",
            "updatedAt": "2006-01-02T15:04:05Z",
            "name": "foo",
            "ingestUrl": "https://in.example.invalid/xyz",
            "type": "generic-webhook",
            "metadata": {}
        }),
        IngestSourceOut {
            created_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            id: "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr".to_owned(),
            ingest_url: Some("https://in.example.invalid/xyz".to_owned()),
            name: "foo".to_owned(),
            uid: None,
            updated_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            config: IngestSourceOutConfig::GenericWebhook,
            metadata: BTreeMap::new(),
        },
    );

    assert_deserializes_to(
        json!({
            "id": "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr",
            "createdAt": "2006-01-02T15:04:05Z",
            "updatedAt": "2006-01-02T15:04:05Z",
            "name": "foo",
            "ingestUrl": "https://in.example.invalid/xyz",
            "type": "svix",
            "config": { "secret": "xxx" },
            "metadata": {}
        }),
        IngestSourceOut {
            created_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            id: "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr".to_owned(),
            ingest_url: Some("https://in.example.invalid/xyz".to_owned()),
            name: "foo".to_owned(),
            uid: None,
            updated_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            config: IngestSourceOutConfig::Svix(SvixConfigOut {}),
            metadata: BTreeMap::new(),
        },
    );

    assert_deserializes_to(
        json!({
            "id": "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr",
            "createdAt": "2006-01-02T15:04:05Z",
            "updatedAt": "2006-01-02T15:04:05Z",
            "name": "foo",
            "ingestUrl": "https://in.example.invalid/xyz",
            "type": "segment",
            "config": {},
            "metadata": {}
        }),
        IngestSourceOut {
            created_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            id: "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr".to_owned(),
            ingest_url: Some("https://in.example.invalid/xyz".to_owned()),
            name: "foo".to_owned(),
            uid: None,
            updated_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            config: IngestSourceOutConfig::Segment(SegmentConfigOut::new()),
            metadata: BTreeMap::new(),
        },
    );

    assert_deserializes_to(
        json!({
            "id": "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr",
            "createdAt": "2006-01-02T15:04:05Z",
            "updatedAt": "2006-01-02T15:04:05Z",
            "name": "foo",
            "type": "cron",
            "config": {
                "payload": "💣",
                "schedule": "* * * * *",
            },
            "metadata": {}
        }),
        IngestSourceOut {
            created_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            id: "Rjb52OFZK6aYPfF4EpqYqD8Ptcyr".to_owned(),
            ingest_url: None,
            name: "foo".to_owned(),
            uid: None,
            updated_at: "2006-01-02T15:04:05Z".parse().unwrap(),
            config: IngestSourceOutConfig::Cron(CronConfig {
                content_type: None,
                payload: "💣".to_owned(),
                schedule: "* * * * *".to_owned(),
            }),
            metadata: BTreeMap::new(),
        },
    );
}

fn assert_deserializes_to<T: Debug + PartialEq + DeserializeOwned>(
    value: serde_json::Value,
    expected: T,
) {
    let actual = T::deserialize(value).unwrap();
    assert_eq!(actual, expected);
}
