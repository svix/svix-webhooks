use anyhow::Result;
use dialoguer::Input;

use crate::{config, config::Config};

pub fn prompt() -> Result<()> {
    print!("Welcome to the Svix CLI!\n\n");

    let auth_token = Input::new()
        .with_prompt("Auth Token")
        .validate_with({
            move |input: &String| -> Result<()> {
                if !input.trim().is_empty() {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("auth token cannot be empty"))
                }
            }
        })
        .interact_text()?
        .trim()
        .to_string();

    // Load from disk and update the prompted fields.
    // There are other fields (not prompted for) related to "relay" for the `listen` command
    // that we'd rather not wipe out if `login` is invoked.
    let mut cfg = Config::load()?;
    cfg.auth_token = Some(auth_token);
    let fp = config::get_config_file_path()?;
    if let Err(e) = cfg.save_to_disk(&fp) {
        eprintln!("\n{e}\n");
        anyhow::bail!(
            "Failed to configure the Svix CLI, please try again or try setting your auth \
             token manually `SVIX_AUTH_TOKEN` environment variable."
        );
    }

    println!(
        "All Set! Your config has been written to `{}`",
        fp.as_os_str().to_str().unwrap_or_default()
    );
    println!("Type `svix --help` to print the Svix CLI documentation!");
    Ok(())
}
