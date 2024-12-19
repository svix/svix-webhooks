use crate::relay::message::{MessageOut, MessageOutEvent, MessageOutStart};
use anyhow::{Context, Result};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use http::{HeaderMap, HeaderName, HeaderValue};
use message::{MessageIn, MessageInEvent};
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinSet;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Bytes;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

mod message;
pub mod token;

// Defaults
const DEFAULT_API_HOST: &str = "api.relay.svix.com";
const API_PREFIX: &str = "api/v1";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const WRITE_WAIT: Duration = Duration::from_secs(10);

/// How often the server sends a ping, expecting a pong from us.
///
/// The client will send a pong automatically, but if we don't see a ping from the server at
/// around this interval, we may need to reconnect.
const SERVER_PING_PERIOD: Duration = Duration::from_secs(
    // The actual frequency seems to be 20s on the dot, but travel time means the message often
    // arrives just a little later.
    21,
);

type HttpClient = reqwest::Client;
type LocalServerResponse = reqwest::Response;

struct Client {
    token: String,
    websocket_url: url::Url,
    local_url: url::Url,
    http_client: HttpClient,
    logging: bool,
}

impl Client {
    async fn connect(&mut self, announce: bool) -> Result<()> {
        let mut set = JoinSet::new();
        let conn = WsConnection::new(&self.websocket_url).await?;
        let (mut ws_tx, mut ws_rx) = conn.stream.split();

        let (remote_tx, remote_rx) = tokio::sync::mpsc::unbounded_channel::<MessageOut>();

        ws_tx
            .send(Message::Binary(
                serde_json::to_vec(&MessageOut::Start {
                    version: message::VERSION,
                    data: MessageOutStart {
                        token: self.token.clone(),
                    },
                })?
                .into(),
            ))
            .await?;
        match ws_rx.next().await {
            None => anyhow::bail!("no response from server for start message"),
            Some(msg) => {
                let data = msg?.into_data();

                let parsed = match serde_json::from_slice::<MessageIn>(&data)? {
                    MessageIn::Start { data, .. } => data,
                    MessageIn::Event { .. } => {
                        panic!("unexpected event message during start handshake")
                    }
                };
                if announce {
                    println!(
                        r#"
Webhook relay is now listening at
{}

All requests on this endpoint will be forwarded to your local URL:
{}
"#,
                        receive_url(&parsed.token),
                        self.local_url,
                    );
                } else {
                    // Shows that a reconnection attempt succeeded after some failing initial attempts.
                    println!("Connected!");
                }
            }
        }

        // TL;DR `--no-logging` is broken the same way here as it was in Go.
        // Setting `--no-logging` gives a 400 response (invalid token) when you send a webhook to
        // Play.
        if self.logging && announce {
            println!(
                r#"
View logs and debug information at
{}
To disable logging, run `svix listen --no-logging`
"#,
                view_url(&self.token)
            );
        }

        set.spawn({
            let local_url = self.local_url.clone();
            let http_client = self.http_client.clone();
            async move {
                read_from_ws_loop(ws_rx, remote_tx, local_url.clone(), http_client.clone())
                    .await
                    .inspect_err(|e| eprintln!("read loop terminated: {e}"))
            }
        });

        set.spawn(async move {
            send_to_ws_loop(remote_rx, ws_tx)
                .await
                .inspect_err(|e| eprintln!("write loop terminated: {e}"))
        });

        // If any task terminates, trash the rest so we can reconnect.
        if set.join_next().await.is_some() {
            set.shutdown().await;
        }

        Ok(())
    }
}

pub async fn listen(
    local_url: url::Url,
    relay_token: String,
    logging: bool,
    relay_debug_url: Option<&str>,
    relay_disable_security: bool,
) -> Result<()> {
    let scheme = if relay_disable_security { "ws" } else { "wss" };
    let api_host = relay_debug_url.unwrap_or(DEFAULT_API_HOST);
    let token = if logging {
        format!("c_{relay_token}")
    } else {
        relay_token
    };

    let websocket_url = format!("{scheme}://{api_host}/{API_PREFIX}/listen/").parse()?;

    let mut client = Client {
        token,
        websocket_url,
        local_url,
        http_client: HttpClient::new(),
        logging,
    };

    const MAX_BACKOFF: Duration = Duration::from_millis(5000);
    let backoff_schedule = [
        Duration::ZERO,
        Duration::from_millis(100),
        Duration::from_millis(1000),
        MAX_BACKOFF,
    ];

    let mut attempt_count = 0;
    let mut last_attempt = Instant::now();

    loop {
        // Any termination Ok or Err... try to reconnect.
        if let Err(e) = client.connect(attempt_count == 0).await {
            eprintln!("Failed to connect to Webhook Relay: {e}");
        } else {
            eprintln!("Failed to connect to Webhook Relay");
        }

        // Reset the backoff schedule if it's been a while since we've seen a disconnect.
        if last_attempt.elapsed() > MAX_BACKOFF * 2 {
            // N.b. attempt_count `0` is special because that's what prompts the printing of a
            // welcome message in `Client::connect`.
            // When we reset here, starting at `0` here will still avoid the
            // re-print because we increment after selecting the sleep duration.
            attempt_count = 0;
        }

        let backoff = *backoff_schedule.get(attempt_count).unwrap_or(&MAX_BACKOFF);
        eprintln!("Reattempting connection in: {}ms", backoff.as_millis());

        attempt_count += 1;
        last_attempt = Instant::now();

        tokio::time::sleep(backoff).await;
    }
}

fn receive_url(token: &str) -> String {
    format!("https://play.svix.com/in/{token}/")
}

fn view_url(token: &str) -> String {
    format!("https://play.svix.com/view/{token}/")
}

type S = WebSocketStream<MaybeTlsStream<TcpStream>>;

struct WsConnection {
    stream: S,
}

impl WsConnection {
    async fn new(websocket_url: &url::Url) -> Result<Self> {
        let request = websocket_url.to_string().into_client_request()?;
        let (stream, _resp) = connect_async(request)
            .await
            .with_context(|| "failed to connect to websocket server")?;

        Ok(Self { stream })
    }
}

async fn read_from_ws_loop(
    mut rx: SplitStream<S>,
    tx: UnboundedSender<MessageOut>,
    local_url: url::Url,
    client: HttpClient,
) -> Result<()> {
    // We expect to see roughly _at least one Ping_ in each `SERVER_PING_PERIOD`.
    // Other messages may arrive ahead of this schedule.
    // Tracking the time each message is received, we can know if the server has been quiet for too
    // long, possibly requiring us to reconnect.
    let mut last_msg = Instant::now();

    loop {
        const REMOTE_SERVER_CLOSED: &str = "remote server closed connection";

        match tokio::time::timeout(SERVER_PING_PERIOD, rx.next()).await {
            Err(_timeout_hit) => {
                // Generous. 1.5x the ping frequency. If we go that long without
                // seeing anything from the server, force a reconnect.
                if last_msg.elapsed() > SERVER_PING_PERIOD + (SERVER_PING_PERIOD / 2) {
                    anyhow::bail!(REMOTE_SERVER_CLOSED);
                }
            }
            // Stream empty/closed
            Ok(None) => break,
            Ok(Some(msg)) => {
                last_msg = Instant::now();

                let data = match msg? {
                    // Control messages.
                    Message::Close(_) => anyhow::bail!(REMOTE_SERVER_CLOSED),
                    Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => continue,

                    // Messages that carry data we care to process.
                    Message::Text(s) => s.into(),
                    Message::Binary(bytes) => bytes,
                };

                handle_incoming_message(client.clone(), data, &local_url, tx.clone()).await;
            }
        }
    }

    Ok(())
}

async fn send_to_ws_loop(
    mut rx: UnboundedReceiver<MessageOut>,
    mut tx: SplitSink<S, Message>,
) -> Result<()> {
    while let Some(msg) = rx.recv().await {
        tokio::time::timeout(
            WRITE_WAIT,
            tx.send(Message::Binary(
                serde_json::to_vec(&msg)
                    .expect("trivial serialization")
                    .into(),
            )),
        )
        .await?
        .context("Websocket write timeout")?;
    }

    Ok(())
}

async fn make_local_request(
    client: HttpClient,
    url: &url::Url,
    data: MessageInEvent,
) -> Result<LocalServerResponse> {
    let method = data.method.parse()?;
    // FIXME: deprecation warning
    #[allow(deprecated)]
    let body = base64::decode(&data.body)?;
    let mut headers = HeaderMap::with_capacity(data.headers.len());
    for (k, v) in &data.headers {
        // FIXME: there's a remark about the Go client freaking out if there's more than one host header set.
        //   Do we care now that we're not using Go? TBD.
        headers.insert(
            HeaderName::try_from(k.as_str())?,
            HeaderValue::try_from(v.as_str())?,
        );
    }
    Ok(client
        .request(method, url.clone())
        .timeout(DEFAULT_TIMEOUT)
        .body(body)
        .headers(headers)
        .send()
        .await?)
}

fn format_resp_headers(headers: &HeaderMap) -> Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    const SKIP_USER_AGENT: HeaderValue = HeaderValue::from_static("Go-http-client/1.1");
    for (k, v) in headers {
        // FIXME: the Go cli used to unset the useragent if it matched the Go http client. Do we need something similar here? idk why this is needed.
        if k == http::header::USER_AGENT && v == SKIP_USER_AGENT {
            continue;
        }
        out.insert(k.to_string(), v.to_str()?.to_string());
    }
    Ok(out)
}

//
async fn handle_incoming_message(
    client: HttpClient,
    bytes: Bytes,
    local_url: &url::Url,
    tx: UnboundedSender<MessageOut>,
) {
    match serde_json::from_slice::<MessageIn>(&bytes) {
        Ok(MessageIn::Event { data, .. }) => {
            let msg_id = data.id.clone();
            println!("<- Forwarding message id={msg_id} to: {local_url}");
            match make_local_request(client, local_url, data).await {
                Err(err) => {
                    eprintln!("Failed to make request to local server: \n{err}");
                }
                Ok(resp) => {
                    if let Err(err) = process_response(msg_id, resp, tx).await {
                        eprintln!("Failed to read response from local server: \n{err}");
                    }
                }
            }
        }
        Ok(MessageIn::Start { .. }) => { /* nothing to do */ }
        Err(_err) => {
            eprintln!("Received invalid webhook message... skipping");
        }
    }
}

async fn process_response(
    id: String,
    resp: LocalServerResponse,
    tx: UnboundedSender<MessageOut>,
) -> Result<()> {
    let status = resp.status().as_u16();
    let headers = format_resp_headers(resp.headers())?;
    #[allow(deprecated)]
    let body = base64::encode(resp.bytes().await?);
    let msg = MessageOut::Event {
        version: message::VERSION,
        data: MessageOutEvent {
            id,
            body,
            headers,
            status,
        },
    };

    println!("-> Received \"{status}\" response from local server, forwarding to webhook sender");
    Ok(tx.send(msg)?)
}
