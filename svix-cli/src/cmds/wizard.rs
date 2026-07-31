use clap::{Args, Subcommand};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct WizardArgs {
    #[command(subcommand)]
    pub command: WizardCommands,
}

/// Guided setup flows for getting started with Svix
#[derive(Subcommand)]
pub enum WizardCommands {
    /// Walk through a guided Svix quickstart
    Quickstart,
}

impl WizardCommands {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            WizardCommands::Quickstart => {}
        }
        Ok(())
    }
}
