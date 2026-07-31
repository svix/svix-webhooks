use clap::{Args, Subcommand};

use crate::{cmds::login, config::Config};

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
            WizardCommands::Quickstart => quickstart().await?,
        }
        Ok(())
    }
}

async fn quickstart() -> anyhow::Result<()> {
    print!("Welcome to the Svix quickstart!\n\n");

    let _cfg = authenticate().await?;

    Ok(())
}

/// Step 1: make sure we have credentials to work with, reusing the `login` flow.
async fn authenticate() -> anyhow::Result<Config> {
    println!("Step 1: Authenticate");

    let cfg = login::ensure_authenticated().await?;
    println!("You're authenticated with Svix.\n");

    Ok(cfg)
}
