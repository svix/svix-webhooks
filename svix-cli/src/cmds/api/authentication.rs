// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct AuthenticationAppPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationAppPortalAccessOptions>
    for svix::api::AuthenticationAppPortalAccessOptions
{
    fn from(value: AuthenticationAppPortalAccessOptions) -> Self {
        let AuthenticationAppPortalAccessOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationExpireAllOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationExpireAllOptions> for svix::api::AuthenticationExpireAllOptions {
    fn from(value: AuthenticationExpireAllOptions) -> Self {
        let AuthenticationExpireAllOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationLogoutOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationLogoutOptions> for svix::api::AuthenticationLogoutOptions {
    fn from(value: AuthenticationLogoutOptions) -> Self {
        let AuthenticationLogoutOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationStreamLogoutOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationStreamLogoutOptions> for svix::api::AuthenticationStreamLogoutOptions {
    fn from(value: AuthenticationStreamLogoutOptions) -> Self {
        let AuthenticationStreamLogoutOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationStreamPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationStreamPortalAccessOptions>
    for svix::api::AuthenticationStreamPortalAccessOptions
{
    fn from(value: AuthenticationStreamPortalAccessOptions) -> Self {
        let AuthenticationStreamPortalAccessOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationStreamExpireAllOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationStreamExpireAllOptions>
    for svix::api::AuthenticationStreamExpireAllOptions
{
    fn from(value: AuthenticationStreamExpireAllOptions) -> Self {
        let AuthenticationStreamExpireAllOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationRotateStreamPollerTokenOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationRotateStreamPollerTokenOptions>
    for svix::api::AuthenticationRotateStreamPollerTokenOptions
{
    fn from(value: AuthenticationRotateStreamPollerTokenOptions) -> Self {
        let AuthenticationRotateStreamPollerTokenOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AuthenticationArgs {
    #[command(subcommand)]
    pub command: AuthenticationCommands,
}

#[derive(Subcommand)]
pub enum AuthenticationCommands {
    /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication app-portal-access app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"application\": {
    \"metadata\": {\"key\": \"...\"},
    \"name\": \"My first application\",
    \"rateLimit\": 123,
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\"
  },
  \"capabilities\": [\"ViewBase\",\"ViewEndpointSecret\"],
  \"expiry\": 123,
  \"featureFlags\": [],
  \"readOnly\": true,
  \"sessionId\": \"user_1FB8\"
}\n\nExample response:
{
  \"token\": \"appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O\",
  \"url\": \"https://app.svix.com/login#key=eyJhcHBJZCI6ICJhcHBfMXRSdFl\"
}\n")]
    AppPortalAccess {
        app_id: String,
        app_portal_access_in: Option<crate::json::JsonOf<AppPortalAccessIn>>,
        #[clap(flatten)]
        options: AuthenticationAppPortalAccessOptions,
    },
    /// Expire all of the tokens associated with a specific application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication expire-all app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"expiry\": 60,
  \"sessionIds\": [\"...\"]
}\n")]
    ExpireAll {
        app_id: String,
        application_token_expire_in: Option<crate::json::JsonOf<ApplicationTokenExpireIn>>,
        #[clap(flatten)]
        options: AuthenticationExpireAllOptions,
    },
    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication logout\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Logout {
        #[clap(flatten)]
        options: AuthenticationLogoutOptions,
    },
    /// Logout a stream token.
    ///
    /// Trying to log out other tokens will fail.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication stream-logout\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    StreamLogout {
        #[clap(flatten)]
        options: AuthenticationStreamLogoutOptions,
    },
    /// Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication stream-portal-access strm_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"expiry\": 123,
  \"featureFlags\": [],
  \"sessionId\": \"user_1FB8\"
}\n\nExample response:
{
  \"token\": \"appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O\",
  \"url\": \"https://app.svix.com/login#key=eyJhcHBJZCI6ICJhcHBfMXRSdFl\"
}\n")]
    StreamPortalAccess {
        stream_id: String,
        stream_portal_access_in: Option<crate::json::JsonOf<StreamPortalAccessIn>>,
        #[clap(flatten)]
        options: AuthenticationStreamPortalAccessOptions,
    },
    /// Expire all of the tokens associated with a specific stream.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication stream-expire-all strm_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"expiry\": 60,
  \"sessionIds\": [\"...\"]
}\n")]
    StreamExpireAll {
        stream_id: String,
        stream_token_expire_in: Option<crate::json::JsonOf<StreamTokenExpireIn>>,
        #[clap(flatten)]
        options: AuthenticationStreamExpireAllOptions,
    },
    /// Get the current auth token for the stream poller.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication get-stream-poller-token strm_abc000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"expiresAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"...\",
  \"name\": \"...\",
  \"scopes\": [\"...\"],
  \"token\": \"...\"
}\n")]
    GetStreamPollerToken { stream_id: String, sink_id: String },
    /// Create a new auth token for the stream poller API.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication rotate-stream-poller-token strm_abc000000000000000000 sink_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"expiry\": 123,
  \"oldTokenExpiry\": 123
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"expiresAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"...\",
  \"name\": \"...\",
  \"scopes\": [\"...\"],
  \"token\": \"...\"
}\n")]
    RotateStreamPollerToken {
        stream_id: String,
        sink_id: String,
        rotate_poller_token_in: Option<crate::json::JsonOf<RotatePollerTokenIn>>,
        #[clap(flatten)]
        options: AuthenticationRotateStreamPollerTokenOptions,
    },
    /// Return information about the account associated with the current token
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix authentication whoami\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"appId\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"envId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"permissionSource\": \"OidcJwt\",
  \"sessionId\": \"user_1FB8\",
  \"streamAppId\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\"
}\n")]
    Whoami {},
}

impl AuthenticationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::AppPortalAccess {
                app_id,
                app_portal_access_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .app_portal_access(
                        app_id,
                        app_portal_access_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpireAll {
                app_id,
                application_token_expire_in,
                options,
            } => {
                client
                    .authentication()
                    .expire_all(
                        app_id,
                        application_token_expire_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
            Self::Logout { options } => {
                client.authentication().logout(Some(options.into())).await?;
            }
            Self::StreamLogout { options } => {
                client
                    .authentication()
                    .stream_logout(Some(options.into()))
                    .await?;
            }
            Self::StreamPortalAccess {
                stream_id,
                stream_portal_access_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .stream_portal_access(
                        stream_id,
                        stream_portal_access_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::StreamExpireAll {
                stream_id,
                stream_token_expire_in,
                options,
            } => {
                client
                    .authentication()
                    .stream_expire_all(
                        stream_id,
                        stream_token_expire_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
            Self::GetStreamPollerToken { stream_id, sink_id } => {
                let resp = client
                    .authentication()
                    .get_stream_poller_token(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateStreamPollerToken {
                stream_id,
                sink_id,
                rotate_poller_token_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .rotate_stream_poller_token(
                        stream_id,
                        sink_id,
                        rotate_poller_token_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Whoami {} => {
                let resp = client.authentication().whoami().await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
