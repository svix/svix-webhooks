use anyhow::Result;
use clap::Args;

use crate::config::Config;

#[derive(Args)]
pub struct ListenArgs {
    /// The local URL to forward webhooks to
    url: url::Url,
    /// Connect to an existing Play token instead of creating a new one
    #[arg(long)]
    token: Option<String>,
    /// Disable TLS certificate verification when connecting to the local URL
    #[arg(long)]
    disable_tls_verification: bool,
}

impl ListenArgs {
    pub async fn exec(self, cfg: &Config) -> Result<()> {
        let (token, allow_token_rotation) = match self.token {
            Some(token) => (token, false),
            None => {
                let token = crate::relay::token::generate_token()?;
                (token, true)
            }
        };
        crate::relay::listen(
            self.url,
            token,
            allow_token_rotation,
            cfg.relay_debug_hostname.as_deref(),
            cfg.relay_disable_security.unwrap_or_default(),
            self.disable_tls_verification,
        )
        .await?;
        Ok(())
    }
}
