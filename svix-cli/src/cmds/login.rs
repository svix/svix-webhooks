use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use dialoguer::Input;
use reqwest::Client;
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
        fp.display()
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiscoverySessionOut {
    pub region: String,
}

const DASHBOARD_URL: &str = "https://dashboard.svix.com";
const LOGIN_SERVER_URL: &str = "https://api.svix.com";

pub async fn dashboard_login() -> Result<String> {
    let client = reqwest::Client::new();

    let start_session = client
        .post(format!("{LOGIN_SERVER_URL}/dashboard/cli/login/start"))
        .send()
        .await
        .context("Failed to get session ID. Could not connect to server.")?
        .json::<CliStartLoginSessionOut>()
        .await
        .context("Failed to get session ID. Invalid response.")?;

    let session_id = start_session.session_id;
    let code = &session_id[0..4].to_uppercase();

    let url = format!("{DASHBOARD_URL}/cli/login?sessionId={session_id}&code={code}");

    println!("\nPlease approve the login in your browser, then return here.");
    println!("Verification code: \x1b[32m{code}\x1b[0m\n");

    if let Err(e) = open::that(&url) {
        eprintln!("Failed to open browser: {e}");
        println!("Please manually open this URL in your browser: {url}");
    }

    println!("Waiting for approval...");

    // First, poll the discovery endpoint to get the region
    let discovery_poll_url = format!("{LOGIN_SERVER_URL}/dashboard/cli/login/discovery/complete");
    let discovery_data: DiscoverySessionOut =
        poll_session(&client, &discovery_poll_url, &session_id).await?;

    let region = discovery_data.region;
    let region_server_url = format!("https://api.{region}.svix.com");
    let token_poll_url = format!("{region_server_url}/dashboard/cli/login/token/complete");

    // Then, poll the token endpoint to get the auth token
    let token_data: AuthTokenOut = poll_session(&client, &token_poll_url, &session_id).await?;

    println!("Authentication successful!\n");
    Ok(token_data.token)
}

const MAX_POLL_TIME: Duration = Duration::from_secs(5 * 60);

async fn poll_session<T>(client: &Client, poll_url: &str, session_id: &str) -> Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let start_time: Instant = Instant::now();

    while start_time.elapsed() < MAX_POLL_TIME {
        let response = client
            .post(poll_url)
            .json(&serde_json::json!({ "sessionId": session_id }))
            .send()
            .await
            .context("Failed to connect to authentication server")?;

        if response.status().is_success() {
            return response
                .json::<T>()
                .await
                .context("Failed to parse authentication data");
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

            anyhow::bail!("Authentication failed: {error_message}");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    anyhow::bail!("Authentication failed.");
}
