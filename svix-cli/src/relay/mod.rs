use std::{
    collections::HashMap,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    time::Duration,
};

use anyhow::{
    Context,
    Result,
};
use futures_util::{
    stream::{
        SplitSink,
        SplitStream,
    },
    SinkExt,
    StreamExt,
};
use http::{
    HeaderMap,
    HeaderName,
    HeaderValue,
};
use indoc::printdoc;
use message::{
    MessageIn,
    MessageInEvent,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    task::JoinSet,
    time::Instant,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        protocol::{
            frame::coding::CloseCode::Policy,
            CloseFrame,
            Message,
        },
        Bytes,
        Utf8Bytes,
    },
    MaybeTlsStream,
    WebSocketStream,
};

use crate::relay::{
    message::{
        MessageOut,
        MessageOutEvent,
        MessageOutStart,
    },
    token::generate_token,
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

/// When multiple clients try to connect to the Relay server using the same token, one will "win"
/// and the others will get a Close frame with this message as the reason.
const SOCKET_IN_USE_REASON: Utf8Bytes = Utf8Bytes::from_static("This socket is already in use");

type HttpClient = reqwest::Client;
type LocalServerResponse = reqwest::Response;

struct Client {
    token: String,
    websocket_url: url::Url,
    local_url: url::Url,
    http_client: HttpClient,
}

/// Special handling for the errors during establishing a websocket connection.
///
/// In a situation where a relay token is already in use, the server will send a `Close` frame.
/// When this happens, the caller of `Client::connect` may want to try again with a different token.
///
/// For all other error cases, we report/propagate in the same way as we ever have.
struct TokenInUse;

impl Debug for TokenInUse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str("TokenInUse")
    }
}

impl Display for TokenInUse {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str("TokenInUse")
    }
}

impl std::error::Error for TokenInUse {}

impl Client {
    async fn connect(
        &mut self,
        show_welcome_message: bool,
    ) -> Result<()> {
        let mut set = JoinSet::new();
        let conn = WsConnection::new(&self.websocket_url).await?;
        let (mut ws_tx, mut ws_rx) = conn.stream.split();

        let (remote_tx, remote_rx) = tokio::sync::mpsc::unbounded_channel::<MessageOut>();

        match tokio::time::timeout(
            WRITE_WAIT,
            ws_tx.send(
                Message::Text(
                    serde_json::to_string(
                        &MessageOut::Start {
                            version: message::VERSION,
                            data: MessageOutStart {
                                token: self.token.clone(),
                            },
                        },
                    )?
                    .into(),
                ),
            ),
        )
        .await
        {
            Ok(Ok(_)) => { /* nothing to do  */ }
            // The outer Result is for the timeout, the inner is for if there was some other failure
            // during `send`.
            Ok(Err(_)) | Err(_) => {
                anyhow::bail!("failed to complete handshake with Webhook Relay server: remote didn't accept start message");
            }
        }

        // The assumption is the very first message we get from the websocket reader will be the
        // response to our `MessageOut::Start` but it could also be any number of control messages.
        // Keep reading until we see a `MessageIn::Start` or give up after some attempts.
        const MAX_ATTEMPTS: u8 = 10;
        let mut attempts = 0;
        let start_response = loop {
            if attempts > MAX_ATTEMPTS {
                anyhow::bail!("failed to complete handshake with Webhook Relay server: no response from remote");
            }
            attempts += 1;

            match tokio::time::timeout(
                SERVER_PING_PERIOD,
                ws_rx.next(),
            )
            .await
            {
                Err(_timeout) => continue,
                Ok(None) => {
                    anyhow::bail!("no response from server for start message");
                }
                Ok(Some(msg)) => {
                    let data = match msg? {
                        // Control messages.
                        Message::Close(Some(CloseFrame {
                            code,
                            reason,
                        })) if code == Policy && reason == SOCKET_IN_USE_REASON => {
                            return Err(TokenInUse.into())
                        }
                        Message::Close(_) => {
                            anyhow::bail!("Relay server refused connection");
                        }
                        Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => continue,

                        // Messages that carry data we care to process.
                        Message::Text(s) => s.into(),
                        Message::Binary(bytes) => bytes,
                    };

                    match serde_json::from_slice::<MessageIn>(&data)? {
                        // This is what we're waiting to see. A `MessageOut::Start` sent to the writer
                        // should result in a `MessageInStart` coming back on the reader.
                        MessageIn::Start {
                            data,
                            ..
                        } => break data,
                        MessageIn::Event {
                            ..
                        } => continue,
                    };
                }
            }
        };

        if show_welcome_message {
            printdoc!(
                r#"

                Webhook Relay is now listening at:
                {}

                All requests on this endpoint will be forwarded to your local URL:
                {}

                View logs and debug information at:
                {}

                "#,
                receive_url(&start_response.token),
                self.local_url,
                view_url(&self.token),
            );
        } else {
            // Shows that a reconnection attempt succeeded after some failing initial attempts.
            println!("Connected!");
        }

        set.spawn(
            {
                let local_url = self.local_url.clone();
                let http_client = self.http_client.clone();
                async move {
                    read_from_ws_loop(
                        ws_rx,
                        remote_tx,
                        local_url.clone(),
                        http_client.clone(),
                    )
                    .await
                    .inspect_err(|e| eprintln!("read loop terminated: {e}"))
                }
            },
        );

        set.spawn(
            async move {
                send_to_ws_loop(
                    remote_rx, ws_tx,
                )
                .await
                .inspect_err(|e| eprintln!("write loop terminated: {e}"))
            },
        );

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
    relay_debug_url: Option<&str>,
    relay_disable_security: bool,
) -> Result<()> {
    let scheme = if relay_disable_security { "ws" } else { "wss" };
    let api_host = relay_debug_url.unwrap_or(DEFAULT_API_HOST);
    let token = format!("c_{relay_token}");

    let websocket_url = format!("{scheme}://{api_host}/{API_PREFIX}/listen/").parse()?;

    let mut client = Client {
        token,
        websocket_url,
        local_url,
        http_client: HttpClient::new(),
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

    // We may ditch this token, generating a new one on the fly, depending on how the server
    // responds when we connect.
    let orig_token = client.token.clone();
    loop {
        // Any termination Ok or Err... try to reconnect.
        let show_welcome_message = attempt_count == 0 || orig_token != client.token;

        if let Err(e) = client.connect(show_welcome_message).await {
            eprintln!("Failed to connect to Webhook Relay: {e}");
            if e.downcast_ref::<TokenInUse>().is_some() {
                eprintln!("Generating a new token for this session.");
                client.token = {
                    let relay_token = generate_token()?;
                    format!("c_{relay_token}")
                };
            }
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
        eprintln!(
            "Reattempting connection in: {}ms",
            backoff.as_millis()
        );

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
            .inspect_err(|e| eprintln!("{e}"))
            .context("failed to connect to websocket server")?;

        Ok(
            Self {
                stream,
            },
        )
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

        match tokio::time::timeout(
            SERVER_PING_PERIOD,
            rx.next(),
        )
        .await
        {
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

                handle_incoming_message(
                    client.clone(),
                    data,
                    &local_url,
                    tx.clone(),
                )
                .await;
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
            tx.send(
                Message::Binary(
                    serde_json::to_vec(&msg)
                        .expect("trivial serialization")
                        .into(),
                ),
            ),
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
    Ok(
        client
            .request(
                method,
                url.clone(),
            )
            .timeout(DEFAULT_TIMEOUT)
            .body(body)
            .headers(headers)
            .send()
            .await?,
    )
}

fn format_resp_headers(headers: &HeaderMap) -> Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    for (k, v) in headers {
        out.insert(
            k.to_string(),
            v.to_str()?.to_string(),
        );
    }
    Ok(out)
}

async fn handle_incoming_message(
    client: HttpClient,
    bytes: Bytes,
    local_url: &url::Url,
    tx: UnboundedSender<MessageOut>,
) {
    match serde_json::from_slice::<MessageIn>(&bytes) {
        Ok(MessageIn::Event {
            data,
            ..
        }) => {
            let msg_id = data.id.clone();
            println!("<- Forwarding message id={msg_id} to: {local_url}");
            match make_local_request(
                client, local_url, data,
            )
            .await
            {
                Err(err) => {
                    eprintln!("Failed to make request to local server: \n{err}");
                }
                Ok(resp) => {
                    if let Err(err) = process_response(
                        msg_id, resp, tx,
                    )
                    .await
                    {
                        eprintln!("Failed to read response from local server: \n{err}");
                    }
                }
            }
        }
        Ok(MessageIn::Start {
            ..
        }) => { /* nothing to do */ }
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
