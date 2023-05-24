pub use async_trait::async_trait;
use serde::Deserialize;
pub use svix;
use svix::api::SvixOptions as _SvixOptions;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Plain old JSON objects are what the transformations expect to receive and produce.
pub type JsObject = serde_json::Map<String, serde_json::Value>;
/// A channel for plugins to send payloads/scripts to for execution.
pub type TransformerTx = mpsc::UnboundedSender<TransformerJob>;
/// The receiver side for transformations. The JS executor reads from this.
pub type TransformerRx = mpsc::UnboundedReceiver<TransformerJob>;
/// A oneshot channel for the JS executor to "publish" return values to once complete.
// FIXME: better error type?
pub type TransformerCallbackTx = oneshot::Sender<Result<JsReturn, ()>>;
/// Used by the caller of the transformer to await the execution's output.
// FIXME: better error type?
pub type TransformerCallbackRx = oneshot::Receiver<Result<JsReturn, ()>>;

/// A transformation job sent to the JS executor.
/// Once the script has been run on the payload, the transformed payload is sent back through the
/// callback channel.
pub struct TransformerJob {
    pub callback_tx: TransformerCallbackTx,
    pub payload: JsObject,
    pub script: String,
}

pub enum JsReturn {
    /// A successfully transformed payload.
    // XXX: not sure if there's a cheaper way to deserialize the output while requiring an Object.
    Object(JsObject),
    /// For cases where the JS script executes successfully but produces an unexpected output.
    Invalid,
}

impl TransformerJob {
    pub fn new(script: String, payload: JsObject) -> (Self, TransformerCallbackRx) {
        let (callback_tx, callback_rx) = oneshot::channel();
        (
            Self {
                payload,
                script,
                callback_tx,
            },
            callback_rx,
        )
    }
}

/// Effectively a black box to the supervisor.
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
    async fn handle(&self, payload: JsObject) -> std::io::Result<()>;
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
