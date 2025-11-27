use anyhow::Context;
use clap::{
    Args,
    Subcommand,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct OpenArgs {
    #[command(subcommand)]
    pub command: OpenCommands,
}

const API_DOCS_URL: &str = "https://api.svix.com/docs";
const DOCS_URL: &str = "https://docs.svix.com/";

/// Quickly open Svix pages in your browser
#[derive(Subcommand)]
pub enum OpenCommands {
    /// Open the Svix API reference
    Api,
    /// Open the Svix documentation
    Docs,
}

impl OpenCommands {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            OpenCommands::Api => open::that(API_DOCS_URL).context(
                format!(
                    "Failed to open link in your default browser.\n
                Try to paste `{API_DOCS_URL}` into your the address bar."
                ),
            )?,
            OpenCommands::Docs => open::that(DOCS_URL).context(
                format!(
                    "Failed to open link in your default browser.\n
                Try to paste `{DOCS_URL}` into your the address bar."
                ),
            )?,
        }
        Ok(())
    }
}
