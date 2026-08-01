use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use dialoguer::Input;
use reqwest::Client;
use serde::Deserialize;

use crate::{config, config::Config};

pub async fn prompt(_cfg: &Config) -> Result<()> {
    print!("Welcome to the Svix CLI!\n\n");

    let auth_token = prompt_for_auth_token().await?;
    let fp = save_auth_token(auth_token)?;

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

/// Asks the user how they'd like to authenticate and returns the resulting auth token.
///
/// Nothing is persisted here, see [`save_auth_token`].
pub async fn prompt_for_auth_token() -> Result<String> {
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

    Ok(auth_token)
}

/// Persists `auth_token` to the config file, returning the path it was written to.
pub fn save_auth_token(auth_token: String) -> Result<PathBuf> {
    // Load from disk and update the prompted fields.
    // There are other fields (not prompted for) related to "relay" for the `listen` command
    // that we'd rather not wipe out if `login` is invoked.
    let mut cfg = Config::load()?;
    cfg.auth_token = Some(auth_token);
    let fp = config::get_config_file_path()?;
    if let Err(e) = cfg.save_to_disk(&fp) {
        eprintln!("\n{e:#}\n");
        anyhow::bail!(
            "Failed to configure the Svix CLI, please try again or try setting your auth \
             token manually `SVIX_AUTH_TOKEN` environment variable."
        );
    }

    Ok(fp)
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
    let mut session = DashboardLogin::start().await?;

    println!("\nPlease approve the login in your browser, then return here.");
    println!("Verification code: \x1b[32m{}\x1b[0m\n", session.code);

    if let Err(e) = open::that(&session.url) {
        eprintln!("Failed to open browser: {e}");
        println!(
            "Please manually open this URL in your browser: {}",
            session.url
        );
    }

    println!("Waiting for approval...");

    let start_time = Instant::now();
    while start_time.elapsed() < MAX_POLL_TIME {
        if let Some(token) = session.poll().await? {
            println!("Authentication successful!\n");
            return Ok(token);
        }
        std::thread::sleep(POLL_INTERVAL);
    }

    anyhow::bail!("Authentication failed.");
}

pub const MAX_POLL_TIME: Duration = Duration::from_secs(5 * 60);
pub const POLL_INTERVAL: Duration = Duration::from_secs(1);

/// A dashboard login waiting to be approved in the browser.
///
/// Polling is split into single attempts so a caller with its own event loop (the wizard)
/// can keep drawing and stay responsive between them.
pub struct DashboardLogin {
    client: Client,
    session_id: String,
    /// The code the user checks against the one shown in the browser.
    pub code: String,
    /// Where the login is approved.
    pub url: String,
    stage: Stage,
}

/// The login is approved in one place and the token issued in the user's own region, so
/// the region has to be discovered before the token can be polled for.
enum Stage {
    Discovery,
    Token { url: String },
}

impl DashboardLogin {
    pub async fn start() -> Result<Self> {
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
        let code = session_id[0..4].to_uppercase();

        Ok(Self {
            url: format!("{DASHBOARD_URL}/cli/login?sessionId={session_id}&code={code}"),
            client,
            session_id,
            code,
            stage: Stage::Discovery,
        })
    }

    /// Makes one polling attempt, returning the auth token once the login is approved.
    pub async fn poll(&mut self) -> Result<Option<String>> {
        match &self.stage {
            Stage::Discovery => {
                let url = format!("{LOGIN_SERVER_URL}/dashboard/cli/login/discovery/complete");
                let Some(discovery) = self.poll_once::<DiscoverySessionOut>(&url).await? else {
                    return Ok(None);
                };

                let region = discovery.region;
                self.stage = Stage::Token {
                    url: format!(
                        "https://api.{region}.svix.com/dashboard/cli/login/token/complete"
                    ),
                };
                Ok(None)
            }
            Stage::Token { url } => {
                let url = url.clone();
                Ok(self
                    .poll_once::<AuthTokenOut>(&url)
                    .await?
                    .map(|token| token.token))
            }
        }
    }

    async fn poll_once<T>(&self, poll_url: &str) -> Result<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = self
            .client
            .post(poll_url)
            .json(&serde_json::json!({ "sessionId": self.session_id }))
            .send()
            .await
            .context("Failed to connect to authentication server")?;

        if response.status().is_success() {
            return response
                .json::<T>()
                .await
                .map(Some)
                .context("Failed to parse authentication data");
        }

        // Not approved yet; anything else means the session exists but is unusable.
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

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
}
