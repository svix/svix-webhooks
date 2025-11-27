use clap::{
    Args,
    Subcommand,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SignatureArgs {
    #[command(subcommand)]
    pub command: SignatureCommands,
}

#[derive(Subcommand)]
pub enum SignatureCommands {
    /// Generate a signature for the specified webhook payload
    Sign {
        #[clap(long)]
        secret: String,
        #[clap(long)]
        msg_id: String,
        #[clap(long)]
        timestamp: i64,
        payload: String,
    },
    /// Verify the signature of a webhook message
    Verify {
        #[clap(long)]
        secret: String,
        #[clap(long)]
        signature: String,
        #[clap(long)]
        msg_id: String,
        #[clap(long)]
        timestamp: i64,
        payload: String,
    },
}

impl SignatureCommands {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            SignatureCommands::Sign {
                secret,
                msg_id,
                timestamp,
                payload,
            } => {
                let webhook = svix::webhooks::Webhook::new(&secret)?;
                let signature = webhook.sign(
                    &msg_id,
                    timestamp,
                    payload.as_bytes(),
                )?;
                println!("{signature}");
            }
            SignatureCommands::Verify {
                secret,
                signature,
                msg_id,
                timestamp,
                payload,
            } => {
                let webhook = svix::webhooks::Webhook::new(&secret)?;
                let mut headers = http::HeaderMap::with_capacity(3);
                headers.insert(
                    "svix-id",
                    msg_id.parse()?,
                );
                headers.insert(
                    "svix-timestamp",
                    timestamp.to_string().parse()?,
                );
                headers.insert(
                    "svix-signature",
                    signature.parse()?,
                );
                webhook.verify_ignoring_timestamp(
                    payload.as_bytes(),
                    &headers,
                )?;
            }
        }
        Ok(())
    }
}
