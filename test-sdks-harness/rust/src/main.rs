use std::time::Duration;

use serde_json::{json, Value};
use svix::autoconfig_consumer::MessagePollerv2ConsumerPollOptions;
use svix::models::{
    DestinationOut, DestinationOutConfig, EndpointIn, SinkInCommon, StartingPosition,
};
use svix::{AutoConfig, AutoConfigConsumer};

struct ScenarioResult {
    pass: bool,
    detail: String,
}

impl ScenarioResult {
    fn pass(detail: impl Into<String>) -> Self {
        Self {
            pass: true,
            detail: detail.into(),
        }
    }

    fn fail(detail: impl Into<String>) -> Self {
        Self {
            pass: false,
            detail: detail.into(),
        }
    }

    fn line(&self, letter: char) -> String {
        format!(
            "{letter}: {} — {}",
            if self.pass { "PASS" } else { "FAIL" },
            self.detail
        )
    }
}

fn endpoint_in(http_url: &str, event_type: &str) -> EndpointIn {
    let mut endpoint = EndpointIn::new(http_url.to_string());
    endpoint.event_types = Some(vec![event_type.to_string()].into_iter().collect());
    endpoint
}

fn sink_in(event_type: &str) -> SinkInCommon {
    let mut sink = SinkInCommon::new();
    sink.event_types = Some([event_type.to_string()].into_iter().collect());
    sink
}

fn poll_opts() -> MessagePollerv2ConsumerPollOptions {
    MessagePollerv2ConsumerPollOptions {
        limit: None,
        lease_duration_ms: Some(2000),
        starting_position: Some(StartingPosition::Earliest),
    }
}

fn err_text(err: &dyn std::fmt::Display) -> String {
    let s = err.to_string();
    if s.contains("501") {
        format!("DIOM_MISSING: {s}")
    } else {
        s
    }
}

async fn send_msg(
    client: &reqwest::Client,
    server_url: &str,
    org_token: &str,
    app_id: &str,
    event_type: &str,
    src: &str,
) -> Result<(), String> {
    let url = format!("{server_url}/api/v1/app/{app_id}/msg/");
    let resp = client
        .post(&url)
        .bearer_auth(org_token)
        .json(&json!({
            "eventType": event_type,
            "payload": { "ok": true, "src": src },
        }))
        .send()
        .await
        .map_err(|e| format!("send msg request failed: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("send msg HTTP {status}: {body}"));
    }
    Ok(())
}

async fn run_http(token: &str, http_url: &str, event_type: &str) -> ScenarioResult {
    let ac = match AutoConfig::new(token.to_string(), endpoint_in(http_url, event_type)) {
        Ok(ac) => ac,
        Err(e) => return ScenarioResult::fail(format!("new: {}", err_text(&e))),
    };
    match ac.subscribe().await {
        Ok(out) => {
            if out.id.is_empty() {
                ScenarioResult::fail("subscribe returned empty id")
            } else if out.url != http_url {
                ScenarioResult::fail(format!("url mismatch: got {}", out.url))
            } else {
                ScenarioResult::pass(format!("id={} url={}", out.id, out.url))
            }
        }
        Err(e) => ScenarioResult::fail(format!("subscribe: {}", err_text(&e))),
    }
}

fn dest_type_note(dest: &DestinationOut) -> &'static str {
    match dest.config {
        DestinationOutConfig::PollingEndpoint => "pollingEndpoint",
        _ => "other",
    }
}

async fn receive_matching(
    consumer: &mut AutoConfigConsumer,
    consumer_id: &str,
    event_type: &str,
    src: &str,
) -> Result<u64, String> {
    for attempt in 1..=10 {
        match consumer
            .receive(consumer_id.to_string(), Some(poll_opts()))
            .await
        {
            Ok(out) => {
                for msg in &out.data {
                    if msg.event_type == event_type {
                        let got_src = msg.payload.get("src").and_then(Value::as_str);
                        if got_src == Some(src) {
                            return Ok(msg.offset);
                        }
                    }
                }
                if attempt == 10 {
                    return Err(format!(
                        "no matching message after 10 polls (last batch {})",
                        out.data.len()
                    ));
                }
            }
            Err(e) => {
                let text = err_text(&e);
                if text.contains("DIOM_MISSING") || attempt == 10 {
                    return Err(format!("receive: {text}"));
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err("no matching message after 10 polls".into())
}

async fn run_poller(
    token: &str,
    event_type: &str,
    consumer_id: &str,
    client: &reqwest::Client,
    server_url: &str,
    org_token: &str,
    app_id: &str,
    src: &str,
) -> ScenarioResult {
    let mut consumer = match AutoConfigConsumer::new(token.to_string(), sink_in(event_type)) {
        Ok(c) => c,
        Err(e) => return ScenarioResult::fail(format!("new: {}", err_text(&e))),
    };

    let dest = match consumer.subscribe().await {
        Ok(d) => d,
        Err(e) => return ScenarioResult::fail(format!("subscribe: {}", err_text(&e))),
    };
    if dest.id.is_empty() {
        return ScenarioResult::fail("subscribe returned empty dest id");
    }

    if let Err(e) = send_msg(client, server_url, org_token, app_id, event_type, src).await {
        return ScenarioResult::fail(e);
    }

    let offset = match receive_matching(&mut consumer, consumer_id, event_type, src).await {
        Ok(v) => v,
        Err(e) => {
            return ScenarioResult::fail(format!(
                "dest={} type={} receive: {e}",
                dest.id,
                dest_type_note(&dest)
            ))
        }
    };

    if let Err(e) = consumer.commit(consumer_id.to_string(), offset, None).await {
        return ScenarioResult::fail(format!(
            "dest={} offset={offset} commit: {}",
            dest.id,
            err_text(&e)
        ));
    }

    match consumer
        .receive(consumer_id.to_string(), Some(poll_opts()))
        .await
    {
        Ok(after) => {
            if after.data.iter().any(|m| m.offset == offset) {
                ScenarioResult::fail(format!(
                    "dest={} committed offset {offset} still returned",
                    dest.id
                ))
            } else {
                ScenarioResult::pass(format!(
                    "dest={} type={} offset={offset}",
                    dest.id,
                    dest_type_note(&dest)
                ))
            }
        }
        Err(e) => ScenarioResult::fail(format!(
            "dest={} offset={offset} post-commit receive: {}",
            dest.id,
            err_text(&e)
        )),
    }
}

async fn run_poller_existing(
    token: &str,
    event_type: &str,
    consumer_id: &str,
    client: &reqwest::Client,
    server_url: &str,
    org_token: &str,
    app_id: &str,
    src: &str,
) -> ScenarioResult {
    let mut binder = match AutoConfigConsumer::new(token.to_string(), sink_in(event_type)) {
        Ok(c) => c,
        Err(e) => return ScenarioResult::fail(format!("new binder: {}", err_text(&e))),
    };
    let dest = match binder.subscribe().await {
        Ok(d) => d,
        Err(e) => return ScenarioResult::fail(format!("subscribe: {}", err_text(&e))),
    };
    if dest.id.is_empty() {
        return ScenarioResult::fail("subscribe returned empty dest id");
    }

    let mut consumer = match AutoConfigConsumer::new(token.to_string(), sink_in(event_type)) {
        Ok(c) => c,
        Err(e) => return ScenarioResult::fail(format!("new receiver: {}", err_text(&e))),
    };

    if let Err(e) = send_msg(client, server_url, org_token, app_id, event_type, src).await {
        return ScenarioResult::fail(e);
    }

    let offset = match receive_matching(&mut consumer, consumer_id, event_type, src).await {
        Ok(v) => v,
        Err(e) => return ScenarioResult::fail(format!("dest={} receive: {e}", dest.id)),
    };

    if let Err(e) = consumer.commit(consumer_id.to_string(), offset, None).await {
        return ScenarioResult::fail(format!(
            "dest={} offset={offset} commit: {}",
            dest.id,
            err_text(&e)
        ));
    }

    match consumer
        .receive(consumer_id.to_string(), Some(poll_opts()))
        .await
    {
        Ok(after) => {
            if after.data.iter().any(|m| m.offset == offset) {
                ScenarioResult::fail(format!("committed offset {offset} still returned"))
            } else {
                ScenarioResult::pass(format!(
                    "dest={} offset={offset} no subscribe on receiver",
                    dest.id
                ))
            }
        }
        Err(e) => ScenarioResult::fail(format!(
            "offset={offset} post-commit receive: {}",
            err_text(&e)
        )),
    }
}

#[tokio::main]
async fn main() {
    let path = std::env::var("AUTOCONFIG_FIXTURES").expect("AUTOCONFIG_FIXTURES");
    let fixtures: Value =
        serde_json::from_str(&std::fs::read_to_string(path).expect("read fixtures.json"))
            .expect("parse fixtures.json");

    let server_url = fixtures["serverUrl"].as_str().unwrap();
    let org_token = fixtures["orgToken"].as_str().unwrap();
    let event_type = fixtures["eventType"].as_str().unwrap();
    let http_url = fixtures["httpUrl"].as_str().unwrap();
    let consumer_id = fixtures["consumerId"].as_str().unwrap();
    let rust = &fixtures["languages"]["rust"];
    let app_id = rust["appId"].as_str().unwrap();

    let client = reqwest::Client::new();

    let a = run_http(rust["v1Http"].as_str().unwrap(), http_url, event_type).await;
    let b = run_http(rust["v2Http"].as_str().unwrap(), http_url, event_type).await;
    let c = run_poller(
        rust["v1Poller"].as_str().unwrap(),
        event_type,
        consumer_id,
        &client,
        server_url,
        org_token,
        app_id,
        "rust-v1",
    )
    .await;
    let d = run_poller(
        rust["v2Poller"].as_str().unwrap(),
        event_type,
        consumer_id,
        &client,
        server_url,
        org_token,
        app_id,
        "rust-v2",
    )
    .await;
    let e = run_poller_existing(
        rust["v2PollerExisting"].as_str().unwrap(),
        event_type,
        consumer_id,
        &client,
        server_url,
        org_token,
        app_id,
        "rust-e",
    )
    .await;

    println!("LANG: rust");
    println!("{}", a.line('A'));
    println!("{}", b.line('B'));
    println!("{}", c.line('C'));
    println!("{}", d.line('D'));
    println!("{}", e.line('E'));
    println!("Notes: path-dep on rust/; EndpointIn url+event_types");
}
