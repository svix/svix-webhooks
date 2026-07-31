use anyhow::Context as _;
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

    match choose_mode()? {
        // The agent installs the Svix skills and drives the rest of the quickstart itself,
        // so there's nothing left for the wizard to do.
        QuickstartMode::Agent => {
            agent_handoff()?;
            return Ok(());
        }
        QuickstartMode::Manual => {}
    }

    Ok(())
}

/// Step 1: make sure we have credentials to work with, reusing the `login` flow.
async fn authenticate() -> anyhow::Result<Config> {
    println!("Step 1: Authenticate");

    let cfg = login::ensure_authenticated().await?;
    println!("You're authenticated with Svix.\n");

    Ok(cfg)
}

/// How the user wants to work through the rest of the quickstart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuickstartMode {
    /// The user walks through the remaining steps themselves.
    Manual,
    /// A coding agent takes over from here.
    Agent,
}

/// Step 2: ask whether the user drives the rest of the quickstart or an agent does.
fn choose_mode() -> anyhow::Result<QuickstartMode> {
    println!("Step 2: Choose how to continue");

    let selections = &[
        "Continue manually (I'll walk through the steps myself)",
        "Continue with an agent (let a coding agent set things up)",
    ];
    let selection = dialoguer::Select::new()
        .with_prompt("How would you like to continue?")
        .items(selections)
        .default(0)
        .interact()?;

    let mode = if selection == 0 {
        QuickstartMode::Manual
    } else {
        QuickstartMode::Agent
    };

    match mode {
        QuickstartMode::Manual => println!("Continuing manually.\n"),
        QuickstartMode::Agent => println!("An agent will take it from here.\n"),
    }

    Ok(mode)
}

/// The skills published at <https://github.com/svix/ai>, installed via the `skills` CLI.
const SKILLS_PACKAGE: &str = "svix/ai";

/// The skills that drive the rest of the quickstart. The `skills` CLI only takes one
/// skill at a time, so these are installed with one command each.
const SKILL_NAMES: &[&str] = &["svix-sending-webhooks", "receiving-webhooks"];

/// What the user asks their agent once the skills are installed.
const AGENT_PROMPT: &str = "Use the Svix skills to set up my Svix integration.";

/// Agent path: install the Svix agent skills and hand the rest of the quickstart to them.
fn agent_handoff() -> anyhow::Result<()> {
    for skill in SKILL_NAMES {
        install_skill(skill)?;
    }

    println!("\nSvix agent skills installed. Ask your coding agent:\n");
    // Green, matching the verification code in the login flow.
    println!("\x1b[32m{AGENT_PROMPT}\x1b[0m\n");
    println!("The skills take it from there.");

    Ok(())
}

/// Installs a single skill from the `svix/ai` package.
fn install_skill(skill: &str) -> anyhow::Result<()> {
    println!("Installing `{skill}` (`npx skills add {SKILLS_PACKAGE} -y --skill {skill}`)...\n");

    // `-y` skips the `skills` CLI prompts, so the skill to install has to be named explicitly.
    let status = std::process::Command::new("npx")
        .args([
            "--yes",
            "skills",
            "add",
            SKILLS_PACKAGE,
            "-y",
            "--skill",
            skill,
        ])
        .status()
        .context(
            "Failed to run `npx`. Install Node.js and try again, or install the skills \
             yourself with `npx skills add svix/ai`.",
        )?;

    if !status.success() {
        anyhow::bail!(
            "`npx skills add {SKILLS_PACKAGE} -y --skill {skill}` failed. Try running it \
             yourself to see what went wrong."
        );
    }

    Ok(())
}