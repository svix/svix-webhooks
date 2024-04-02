pub use async_trait::async_trait;
use serde::{Deserialize, Serialize};
pub use svix;
use svix::api::{MessageIn, PostOptions as PostOptions_, SvixOptions as _SvixOptions};
use tokio::sync::{mpsc, oneshot};

#[derive(Deserialize, Default, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransformerInputFormat {
    String,
    #[default]
    Json,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum TransformationConfig {
    /// If the config has a string value, we assume it expects the input parsed as json
    /// ```yaml
    /// transformation: function handler(x) {return { payload: x.foobar }; }
    /// ```
    ImplicitJson(String),
    /// When the config has format/src fields, then you can optionally set the format to `string`,
    /// in which case you have to parse it yourself inside the transformation.
    /// ```yaml
    /// transformation:
    ///   format: string
    ///   src: function handler(x) { return { payload: JSON.parse(x).foobar }; }
    /// ```
    Explicit {
        format: TransformerInputFormat,
        src: String,
    },
}

impl TransformationConfig {
    pub fn source(&self) -> &String {
        match self {
            TransformationConfig::ImplicitJson(src) => src,
            TransformationConfig::Explicit { src, .. } => src,
        }
    }

    pub fn format(&self) -> TransformerInputFormat {
        match self {
            TransformationConfig::ImplicitJson(_) => TransformerInputFormat::Json,
            TransformationConfig::Explicit { format, .. } => *format,
        }
    }
}

impl<S> From<S> for TransformationConfig
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self::ImplicitJson(value.into())
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum TransformerInput {
    /// Transformations accept arbitrary json here, not restricted to an Object type.
    /// The thing receiving the value will error if it can't marshall into a type it needs.
    JSON(serde_json::Value),
    /// Aka "raw", we take the input as a utf-8 string and the transformation does whatever it
    /// wants with it.
    String(String),
}

impl From<serde_json::Value> for TransformerInput {
    fn from(value: serde_json::Value) -> Self {
        Self::JSON(value)
    }
}

impl From<String> for TransformerInput {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

/// Plain old JSON objects are what the transformations expect to receive and produce.
pub type JsObject = serde_json::Map<String, serde_json::Value>;
/// A channel for plugins to send payloads/scripts to for execution.
pub type TransformerTx = mpsc::UnboundedSender<TransformerJob>;
/// The receiver side for transformations. The JS executor reads from this.
pub type TransformerRx = mpsc::UnboundedReceiver<TransformerJob>;
/// A oneshot channel for the JS executor to "publish" return values to once complete.
// FIXME: better error type?
pub type TransformerCallbackTx = oneshot::Sender<Result<TransformerOutput, ()>>;
/// Used by the caller of the transformer to await the execution's output.
// FIXME: better error type?
pub type TransformerCallbackRx = oneshot::Receiver<Result<TransformerOutput, ()>>;

/// A transformation job sent to the JS executor.
/// Once the script has been run on the payload, the transformed payload is sent back through the
/// callback channel.
pub struct TransformerJob {
    pub callback_tx: TransformerCallbackTx,
    pub input: TransformerInput,
    pub script: String,
}

#[derive(Debug)]
pub enum TransformerOutput {
    /// A successfully transformed payload.
    // Both senders and receivers require a map type (Object) but have different requirements which
    // are best validated after the fact. For now, we validate only that we get a map type back.
    Object(JsObject),
    /// For cases where the JS script executes successfully but produces an unexpected output.
    Invalid,
}

impl TransformerJob {
    pub fn new(script: String, input: TransformerInput) -> (Self, TransformerCallbackRx) {
        let (callback_tx, callback_rx) = oneshot::channel();
        (
            Self {
                input,
                script,
                callback_tx,
            },
            callback_rx,
        )
    }
}

/// Effectively a black box to the supervisor.
///
/// Plugins should run until they are done, and likely they should not be "done" until the program
/// exits.
#[async_trait]
pub trait SenderInput: Send {
    fn name(&self) -> &str;
    /// For plugins that want to run JS transformations on payloads.
    /// Giving them a sender lets them pass messages to the JS executor.
    fn set_transformer(&mut self, _tx: Option<TransformerTx>) {}
    async fn run(&self) -> std::io::Result<()>;
}

/// Represents something we can hand a webhook payload to.
/// Aka a "forwarder."
///
/// To start, we're only using this in conjunction with an HTTP server "owned" by the bridge binary.
#[async_trait]
pub trait ReceiverOutput: Send + Sync {
    fn name(&self) -> &str;
    async fn handle(&self, request: ForwardRequest) -> std::io::Result<()>;
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WebhookVerifier {
    Svix {
        endpoint_secret: String,
    },
    #[default]
    None,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ReceiverInputOpts {
    Webhook {
        path_id: String,
        #[serde(default)]
        verification: WebhookVerifier,
    },
    #[serde(rename = "svix-webhook")]
    SvixWebhook {
        path_id: String,
        endpoint_secret: String,
    },
}

impl ReceiverInputOpts {
    pub fn path_id(&self) -> &str {
        match self {
            ReceiverInputOpts::Webhook { path_id, .. }
            | ReceiverInputOpts::SvixWebhook { path_id, .. } => path_id,
        }
    }
}

// N.b. the codegen types we get from openapi don't impl Deserialize so we need our own version.
#[derive(Debug, Default, Deserialize)]
pub struct SvixOptions {
    #[serde(default)]
    pub debug: bool,
    pub server_url: Option<String>,
}

impl From<SvixOptions> for _SvixOptions {
    fn from(SvixOptions { debug, server_url }: SvixOptions) -> Self {
        _SvixOptions { debug, server_url }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SenderOutputOpts {
    Svix(SvixSenderOutputOpts),
}

#[derive(Debug, Deserialize)]
pub struct SvixSenderOutputOpts {
    /// Svix API token for the client.
    pub token: String,
    /// Options for the Svix client.
    #[serde(default)]
    pub options: Option<SvixOptions>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct PostOptions {
    idempotency_key: Option<String>,
}

impl From<PostOptions> for PostOptions_ {
    fn from(value: PostOptions) -> Self {
        PostOptions_ {
            idempotency_key: value.idempotency_key,
        }
    }
}

/// Senders convert messages into Create Message API calls so the JSON pulled out of message queues
/// or produced by transformations need to conform to this shape.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
    pub app_id: String,
    pub message: MessageIn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_options: Option<PostOptions>,
}

/// Receivers convert HTTP bodies into messages forwarded to (currently only) message queues, etc.
/// The `payload` field represents the message body given to the producer, and other fields may be
/// added in the future allowing transformations to dynamically customize the producer behavior.
#[derive(Clone, Deserialize, Serialize)]
pub struct ForwardRequest {
    /// This is the payload that will be fed into a Receiver Output
    // XXX: right now I think any arbitrary json value can work, but individual outputs may have
    // more strict requirements.
    // The fact this is represented as a field on a json object demands at least that the value can
    // be represented in json.
    // FIXME: can we leverage RawValue here?
    pub payload: serde_json::Value,
}
