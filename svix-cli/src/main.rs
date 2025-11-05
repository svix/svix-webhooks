use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use colored_json::{ColorMode, Output};
use concolor_clap::{Color, ColorChoice};
use svix::api::SvixOptions;

use self::{
    cmds::{
        api::{
            application::ApplicationArgs, authentication::AuthenticationArgs,
            endpoint::EndpointArgs, environment::EnvironmentArgs, event_type::EventTypeArgs,
            ingest::IngestArgs, integration::IntegrationArgs, message::MessageArgs,
            message_attempt::MessageAttemptArgs, operational_webhook::OperationalWebhookArgs,
            streaming::StreamingArgs,
        },
        listen::ListenArgs,
        open::OpenArgs,
        seed::SeedArgs,
        signature::SignatureArgs,
    },
    config::Config,
};
use crate::cmds::api::connector::ConnectorArgs;

mod cmds;
mod config;
mod json;
mod relay;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Parser)]
#[command(version, about, long_about = None, bin_name = BIN_NAME)]
#[clap(color = concolor_clap::color_choice())]
struct Cli {
    #[command(flatten)]
    color: Color,
    #[arg(
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Log more. This option may be repeated up to 3 times"
    )]
    verbose: u8,
    #[command(subcommand)]
    command: RootCommands,
}

impl Cli {
    /// Converts the selected `ColorChoice` from the CLI to a `ColorMode` as used by the JSON printer.
    ///
    /// When the color choice is "auto", this considers whether stdout is a tty or not so that
    /// color codes are only produced when actually writing directly to a terminal.
    fn color_mode(&self) -> ColorMode {
        match self.color.color {
            ColorChoice::Auto => ColorMode::Auto(Output::StdOut),
            ColorChoice::Always => ColorMode::On,
            ColorChoice::Never => ColorMode::Off,
        }
    }

    fn log_level(&self) -> tracing::Level {
        match self.verbose {
            3.. => tracing::Level::TRACE,
            2 => tracing::Level::DEBUG,
            1 => tracing::Level::INFO,
            0 => tracing::Level::WARN,
        }
    }
}

// N.b Ordering matters here for how clap presents the help.
#[derive(Subcommand)]
enum RootCommands {
    /// List, create & modify applications
    Application(ApplicationArgs),
    /// Manage authentication tasks such as getting dashboard URLs
    Authentication(AuthenticationArgs),
    /// Generate the autocompletion script for the specified shell
    Completion { shell: Shell },
    /// List, create & modify connectors
    Connector(ConnectorArgs),
    /// List, create & modify endpoints
    Endpoint(EndpointArgs),
    /// Import or export environments
    Environment(EnvironmentArgs),
    /// List, create & modify event types
    EventType(EventTypeArgs),
    /// List, create & modify Svix Ingest sources and endpoints
    Ingest(IngestArgs),
    /// List integrations by app id
    Integration(IntegrationArgs),
    /// Forward webhook requests to a local url
    Listen(ListenArgs),
    /// Interactively configure your Svix API credentials
    Login,
    /// List & create messages
    Message(MessageArgs),
    /// List, lookup & resend message attempts
    MessageAttempt(MessageAttemptArgs),
    /// Quickly open Svix pages in your browser
    Open(OpenArgs),
    /// List, create & modify operational webhook endpoints
    OperationalWebhook(OperationalWebhookArgs),
    /// List, create & modify Svix Stream resources
    Streaming(StreamingArgs),
    /// Generate a test application with sample endpoints and event types
    Seed(SeedArgs),
    /// Verifying and signing webhooks with the Svix signature scheme
    Signature(SignatureArgs),
    /// Get the version of the Svix CLI
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let color_mode = cli.color_mode();

    tracing_subscriber::fmt()
        .with_max_level(cli.log_level())
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .init();

    // rustls requires a crypto backend ("provider") choice to be made explicitly
    // The Svix SDK uses the default provider if a default is not installed, but
    // we use reqwest directly in some code paths, which does not do this.
    _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    // XXX: cfg can give an Err in certain situations.
    // Assigning the variable here since several match arms need a `&Config` but the rest of them
    // won't care/are still usable if the config doesn't exist.
    // To this, the `?` is deferred until the point inside a given match arm needs the config value.
    let cfg = Config::load();
    match cli.command {
        // Local-only things
        RootCommands::Version => println!("{VERSION}"),
        RootCommands::Signature(args) => args.command.exec().await?,
        RootCommands::Open(args) => args.command.exec().await?,
        // Remote API calls
        RootCommands::Application(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Authentication(args) => {
            let cfg = cfg?;
            let client = get_client(&cfg)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Connector(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::EventType(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Endpoint(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Environment(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Message(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::MessageAttempt(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Ingest(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Integration(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::OperationalWebhook(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Streaming(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }

        RootCommands::Listen(args) => args.exec(&cfg?).await?,
        RootCommands::Login => cmds::login::prompt(&cfg?).await?,
        RootCommands::Completion { shell } => cmds::completion::generate(&shell)?,
        RootCommands::Seed(args) => {
            let client = get_client(&cfg?)?;
            cmds::seed::exec(&client, args, color_mode).await?;
        }
    }

    Ok(())
}

fn get_client(cfg: &Config) -> Result<svix::api::Svix> {
    let token = cfg.auth_token.clone().ok_or_else(|| {
        anyhow::anyhow!("No auth token set. Try running `{BIN_NAME} login` to get started.")
    })?;
    let opts = get_client_options(cfg)?;
    Ok(svix::api::Svix::new(token, Some(opts)))
}

fn get_client_options(cfg: &Config) -> Result<svix::api::SvixOptions> {
    Ok(svix::api::SvixOptions {
        debug: false,
        server_url: cfg.server_url().map(Into::into),
        timeout: None,
        ..SvixOptions::default()
    })
}
