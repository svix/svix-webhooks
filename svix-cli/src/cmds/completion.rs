use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{
    generate as generate_,
    shells,
    Shell,
};

use crate::BIN_NAME;

pub fn generate(shell: &Shell) -> Result<()> {
    let mut writer = std::io::stdout().lock();
    let mut cmd = crate::Cli::command();
    match shell {
        Shell::Bash => generate_(
            shells::Bash,
            &mut cmd,
            BIN_NAME,
            &mut writer,
        ),
        Shell::Elvish => generate_(
            shells::Elvish,
            &mut cmd,
            BIN_NAME,
            &mut writer,
        ),
        Shell::Fish => generate_(
            shells::Fish,
            &mut cmd,
            BIN_NAME,
            &mut writer,
        ),
        Shell::PowerShell => generate_(
            shells::PowerShell,
            &mut cmd,
            BIN_NAME,
            &mut writer,
        ),
        Shell::Zsh => generate_(
            shells::Zsh,
            &mut cmd,
            BIN_NAME,
            &mut writer,
        ),
        _ => anyhow::bail!("unsupported shell"),
    }
    Ok(())
}
