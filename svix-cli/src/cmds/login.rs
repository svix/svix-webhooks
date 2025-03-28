use std::time::{Duration, Instant};

use anyhow::Result;
use dialoguer::Input;
use serde::Deserialize;

use crate::{config, config::Config};

pub async fn prompt(_cfg: &Config) -> Result<()> {
    print!("Welcome to the Svix CLI!\n\n");

    let selections = &["Login in dashboard.svix.com", "Input token manually"];
    let selection = dialoguer::Select::new()
        .with_prompt("How would you like to authenticate?")
        .items(selections)
        .default(0)
        .interact()?;

    let auth_token = if selection == 0 {
        dashboard_login().await?
    } else {
        Input::new()
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
            .to_string()
    };

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
    println!(
        "Type `{} --help` to print the Svix CLI documentation!",
        crate::BIN_NAME
    );
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CliStartLoginSessionOut {
    session_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthTokenOut {
    token: String,
}

const DASHBOARD_URL: &str = "https://dashboard.svix.com";

// Always use EU for the login session. The dashboard UI will do the same.
const LOGIN_SERVER_URL: &str = "https://api.eu.svix.com";

const MAX_POLL_TIME: Duration = Duration::from_secs(5 * 60);

pub async fn dashboard_login() -> Result<String> {
    let client = reqwest::Client::new();

    let start_session = match client
        .post(format!(
            "{LOGIN_SERVER_URL}/dashboard/cli/login/session/start"
        ))
        .send()
        .await
    {
        Ok(res) => res.json::<CliStartLoginSessionOut>().await?,
        Err(e) => {
            eprintln!("Failed to get session ID: {}", e);
            return Err(anyhow::anyhow!("Failed to get session ID"));
        }
    };
    let session_id = start_session.session_id;
    let code = &session_id[0..4];

    let url = format!("{DASHBOARD_URL}/cli/login?sessionId={session_id}&code={code}");

    println!("\nPlease approve the login in your browser, then return here.");
    println!("Verification code: \x1b[32m{}\x1b[0m\n", code);

    if let Err(e) = open::that(&url) {
        eprintln!("Failed to open browser: {}", e);
        println!("Please manually open this URL in your browser: {}", url);
    }

    println!("Waiting for approval...");

    let poll_url = format!("{LOGIN_SERVER_URL}/dashboard/cli/login/session/complete");

    let start_time = Instant::now();

    while start_time.elapsed() < MAX_POLL_TIME {
        let response = match client
            .post(&poll_url)
            .json(&serde_json::json!({ "sessionId": session_id }))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to connect to authentication server: {}",
                    e
                ));
            }
        };

        if response.status().is_success() {
            match response.json::<AuthTokenOut>().await {
                Ok(data) => {
                    println!("Authentication successful!\n");
                    return Ok(data.token);
                }
                Err(_) => return Err(anyhow::anyhow!("Failed to parse auth token")),
            }
        } else if response.status() != reqwest::StatusCode::NOT_FOUND {
            // Bail if session exists but has an error (is expired or something else)
            let error_message = match response.json::<serde_json::Value>().await {
                Ok(json) => json
                    .get("detail")
                    .and_then(|d| d.as_str())
                    .unwrap_or("Unknown error")
                    .to_string(),
                Err(_) => "Unknown error".to_string(),
            };

            return Err(anyhow::anyhow!("Authentication failed: {}", error_message));
        }

        // NOT_FOUND case - keep polling
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    anyhow::bail!("Authentication timed out. Please try again.");
}
