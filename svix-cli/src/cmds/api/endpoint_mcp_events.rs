// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct EndpointMcpEventsSubscribeOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointMcpEventsSubscribeOptions> for svix::api::EndpointMcpEventsSubscribeOptions {
    fn from(value: EndpointMcpEventsSubscribeOptions) -> Self {
        let EndpointMcpEventsSubscribeOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointMcpEventsUnsubscribeOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointMcpEventsUnsubscribeOptions> for svix::api::EndpointMcpEventsUnsubscribeOptions {
    fn from(value: EndpointMcpEventsUnsubscribeOptions) -> Self {
        let EndpointMcpEventsUnsubscribeOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EndpointMcpEventsArgs {
    #[command(subcommand)]
    pub command: EndpointMcpEventsCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum EndpointMcpEventsCommands {
    /// Create or refresh an MCP Events subscription.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix endpoint mcp-events subscribe app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"user.signup\",
  \"arguments\": {\"key\": \"...\"},
  \"delivery\": {
    \"mode\": \"webhook\",
    \"url\": \"...\",
    \"secret\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
  },
  \"ttlMs\": 123,
  \"secretGracePeriodSeconds\": 123,
  \"challengeMode\": \"challenge-response\",
  \"channels\": [\"...\"]
}\n\nExample response:
{
  \"id\": \"unique-identifier\",
  \"endpointId\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"refreshBefore\": \"2030-01-01T00:00:00Z\",
  \"cursor\": \"...\",
  \"truncated\": true
}\n")]
    Subscribe {
        app_id: String,
        mcp_events_subscribe_in: crate::json::JsonOf<McpEventsSubscribeIn>,
        #[clap(flatten)]
        options: EndpointMcpEventsSubscribeOptions,
    },
    /// Stop an MCP Events subscription.
    ///
    /// Forward an MCP server's `events/unsubscribe` requests here. The
    /// subscription's endpoint is deleted. Unsubscribing from a subscription that
    /// doesn't exist succeeds, so this can be repeated.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix endpoint mcp-events unsubscribe app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"user.signup\",
  \"arguments\": {\"key\": \"...\"},
  \"delivery\": {
    \"mode\": \"webhook\",
    \"url\": \"...\"
  }
}\n")]
    Unsubscribe {
        app_id: String,
        mcp_events_unsubscribe_in: crate::json::JsonOf<McpEventsUnsubscribeIn>,
        #[clap(flatten)]
        options: EndpointMcpEventsUnsubscribeOptions,
    },
}

impl EndpointMcpEventsCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Subscribe {
                app_id,
                mcp_events_subscribe_in,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .mcp_events()
                    .subscribe(
                        app_id,
                        mcp_events_subscribe_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Unsubscribe {
                app_id,
                mcp_events_unsubscribe_in,
                options,
            } => {
                client
                    .endpoint()
                    .mcp_events()
                    .unsubscribe(
                        app_id,
                        mcp_events_unsubscribe_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
