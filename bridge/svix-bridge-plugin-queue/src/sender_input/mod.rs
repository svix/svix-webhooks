use omniqueue::DynConsumer;
use svix_bridge_types::{
    async_trait, svix::api::Svix, SenderInput, SenderOutputOpts, TransformationConfig,
    TransformerTx,
};

use crate::{config::QueueInputOpts, gcp_pubsub, rabbitmq, run_inner, sqs, Consumer};

pub struct QueueSender {
    name: String,
    source: String,
    system: String,
    input_opts: QueueInputOpts,
    transformation: Option<TransformationConfig>,
    transformer_tx: Option<TransformerTx>,
    svix_client: Svix,
}

impl std::fmt::Debug for QueueSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SenderInput").finish()
    }
}

fn system_name(opts: &QueueInputOpts) -> &'static str {
    match opts {
        QueueInputOpts::GcpPubSub(_) => "gcp-pubsub",
        QueueInputOpts::RabbitMQ(_) => "rabbitmq",
        QueueInputOpts::Redis(_) => "redis",
        QueueInputOpts::Sqs(_) => "sqs",
    }
}

fn source_name(opts: &QueueInputOpts) -> &str {
    match opts {
        QueueInputOpts::GcpPubSub(opts) => &opts.subscription_id,
        QueueInputOpts::RabbitMQ(opts) => &opts.queue_name,
        QueueInputOpts::Redis(opts) => &opts.queue_key,
        QueueInputOpts::Sqs(opts) => &opts.queue_dsn,
    }
}

impl QueueSender {
    pub fn new(
        name: String,
        input: QueueInputOpts,
        transformation: Option<TransformationConfig>,
        output: SenderOutputOpts,
    ) -> Self {
        Self {
            name,
            source: source_name(&input).into(),
            system: system_name(&input).into(),
            input_opts: input,
            transformation,
            transformer_tx: None,
            svix_client: match output {
                SenderOutputOpts::Svix(output) => {
                    Svix::new(output.token, output.options.map(Into::into))
                }
            },
        }
    }
}

#[async_trait]
impl Consumer for QueueSender {
    fn source(&self) -> &str {
        &self.source
    }

    fn system(&self) -> &str {
        &self.system
    }

    fn transformer_tx(&self) -> Option<&TransformerTx> {
        self.transformer_tx.as_ref()
    }

    fn transformation(&self) -> Option<&TransformationConfig> {
        self.transformation.as_ref()
    }

    fn svix_client(&self) -> &Svix {
        &self.svix_client
    }

    async fn consumer(&self) -> std::io::Result<DynConsumer> {
        Ok(match &self.input_opts {
            QueueInputOpts::GcpPubSub(cfg) => gcp_pubsub::consumer(cfg).await?,
            QueueInputOpts::RabbitMQ(cfg) => rabbitmq::consumer(cfg).await?,
            QueueInputOpts::Redis(cfg) => crate::redis::consumer(cfg).await?,
            QueueInputOpts::Sqs(cfg) => sqs::consumer(cfg).await?,
        })
    }
}

#[async_trait]
impl SenderInput for QueueSender {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }

    async fn run(&self) {
        run_inner(self).await
    }
}
