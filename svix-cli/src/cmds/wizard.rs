use std::collections::BTreeSet;

use anyhow::Context as _;
use clap::{Args, Subcommand};
use svix::{
    api::Svix,
    models::{ApplicationIn, ApplicationOut, EndpointIn, EndpointOut, EventTypeIn},
};

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

    let cfg = authenticate().await?;

    match choose_mode()? {
        // The agent installs the Svix skills and drives the rest of the quickstart itself,
        // so there's nothing left for the wizard to do.
        QuickstartMode::Agent => {
            agent_handoff()?;
            return Ok(());
        }
        QuickstartMode::Manual => {}
    }

    let client = crate::get_client(&cfg)?;

    // The event type has to exist before the endpoint that filters on it is created.
    let message = sample_message(&client).await?;
    let app = create_application(&client, &message).await?;
    integrate_code(&cfg, &app, &message)?;

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

/// Wraps `text` in the green used elsewhere in the CLI for values worth copying.
fn green(text: &str) -> String {
    format!("\x1b[32m{text}\x1b[0m")
}

/// Svix Play, the throwaway inbox the dashboard onboarding also delivers to.
const PLAY_URL: &str = "https://play.svix.com";

/// Play tokens are 27 base62 characters, same as the dashboard generates.
const PLAY_TOKEN_LEN: usize = 27;
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn generate_token(len: usize) -> String {
    use rand::Rng as _;

    let mut rng = rand::rng();
    (0..len)
        .map(|_| BASE62[rng.random_range(0..BASE62.len())] as char)
        .collect()
}

fn generate_play_token() -> String {
    generate_token(PLAY_TOKEN_LEN)
}

/// The application the quickstart creates. The uid gets a random suffix so repeated runs
/// don't collide.
const APP_NAME: &str = "My first app";
const APP_UID_PREFIX: &str = "quickstart-";

/// The channel used when the org requires one, same as the dashboard onboarding.
const CHANNEL: &str = "my-channel";

/// Step 3 (manual): create the application and give it somewhere to deliver to.
///
/// This mirrors the dashboard onboarding: an application plus a Svix Play endpoint, so
/// the first message has a destination without the user having to run a server.
async fn create_application(client: &Svix, msg: &SampleMessage) -> anyhow::Result<ApplicationOut> {
    println!("Step 3: Create a consumer application");
    println!(
        "A consumer application defines where your messages are sent. Usually you'll want \
         \none application for each of your customers.\n"
    );

    // Named and uid'd for you: in a real integration these come from your own user or
    // tenant record, not from a prompt.
    let application_in = ApplicationIn {
        uid: Some(format!("{APP_UID_PREFIX}{}", generate_token(8))),
        ..ApplicationIn::new(APP_NAME.to_owned())
    };
    let app = client
        .application()
        .create(application_in, None)
        .await
        .context("Failed to create the application")?;

    println!("Created application \"{APP_NAME}\" ({})", green(&app.id));
    if let Some(uid) = &app.uid {
        println!(
            "Its uid is {}, so you can address it by either.",
            green(uid)
        );
    }

    // The dashboard onboarding adds this endpoint for you too, so there's something to
    // deliver to before the user has an endpoint of their own.
    let play_token = generate_play_token();
    let (endpoint, channel) = create_play_endpoint(client, &app.id, &play_token, msg).await?;

    println!(
        "Added an example endpoint ({}) pointing at a Svix Play inbox, so the message \
         \nyou send next has somewhere to go.\n",
        green(&endpoint.id)
    );

    println!(
        "Anything you send to it lands in a Svix Play inbox:\n{}\n",
        green(&format!("{PLAY_URL}/view/{play_token}/"))
    );
    if let Some(channel) = &channel {
        println!("The endpoint only listens on the `{channel}` channel.\n");
    }

    Ok(app)
}

/// Creates the Svix Play endpoint, returning it and the channel it listens on (if any).
///
/// Orgs can require every endpoint to specify filter types and/or channels. Filter types
/// are always sent (the endpoint only needs the one event type the quickstart uses); the
/// channel is only added if the API rejects the first attempt for the lack of one, since
/// the public API doesn't expose those org settings the way the dashboard reads them.
async fn create_play_endpoint(
    client: &Svix,
    app_id: &str,
    play_token: &str,
    msg: &SampleMessage,
) -> anyhow::Result<(EndpointOut, Option<String>)> {
    let endpoint_in = EndpointIn {
        description: Some("Svix onboarding endpoint".to_owned()),
        // Serialized as `filterTypes`; required when the org sets `requireEndpointFilterTypes`.
        event_types: Some(BTreeSet::from([msg.event_type.clone()])),
        ..EndpointIn::new(format!("{PLAY_URL}/in/{play_token}/"))
    };

    match client
        .endpoint()
        .create(app_id.to_owned(), endpoint_in.clone(), None)
        .await
    {
        Ok(endpoint) => Ok((endpoint, None)),
        Err(e) if is_missing_field(&e, "channels") => {
            let endpoint_in = EndpointIn {
                channels: Some(BTreeSet::from([CHANNEL.to_owned()])),
                ..endpoint_in
            };
            let endpoint = client
                .endpoint()
                .create(app_id.to_owned(), endpoint_in, None)
                .await
                .context("Failed to create the example endpoint")?;

            Ok((endpoint, Some(CHANNEL.to_owned())))
        }
        Err(e) => Err(anyhow::Error::new(e).context("Failed to create the example endpoint")),
    }
}

/// Whether `err` is a validation error complaining about the given request body field.
fn is_missing_field(err: &svix::error::Error, field: &str) -> bool {
    let svix::error::Error::Validation(content) = err else {
        return false;
    };

    content.payload.as_ref().is_some_and(|payload| {
        payload
            .detail
            .iter()
            .any(|item| item.loc.iter().any(|loc| loc == field))
    })
}

/// The message the quickstart sends, and that the code snippets show.
struct SampleMessage {
    event_type: String,
    payload: serde_json::Value,
}

/// Fallbacks matching the dashboard onboarding when the account has no event types yet.
const DEFAULT_EVENT_TYPE: &str = "invoice.paid";

fn default_payload() -> serde_json::Value {
    serde_json::json!({ "id": "invoice_WF7WtC", "status": "paid", "attempt": 1 })
}

/// Picks the message to send: the account's first event type if it has one, otherwise
/// `invoice.paid`, which is created here so the endpoint can filter on it.
async fn sample_message(client: &Svix) -> anyhow::Result<SampleMessage> {
    let existing = client
        .event_type()
        .list(None)
        .await
        .ok()
        .and_then(|res| res.data.into_iter().next())
        .map(|et| et.name);

    let event_type = match existing {
        Some(event_type) => event_type,
        None => {
            let event_type_in = EventTypeIn {
                name: DEFAULT_EVENT_TYPE.to_owned(),
                description: "An invoice was paid".to_owned(),
                schemas: None,
                archived: None,
                deprecated: None,
                feature_flags: None,
                group_name: None,
            };
            client
                .event_type()
                .create(event_type_in, None)
                .await
                .with_context(|| {
                    format!("Failed to create the `{DEFAULT_EVENT_TYPE}` event type")
                })?;

            println!("Created event type {}\n", green(DEFAULT_EVENT_TYPE));
            DEFAULT_EVENT_TYPE.to_owned()
        }
    };

    Ok(SampleMessage {
        event_type,
        payload: default_payload(),
    })
}

/// A language the quickstart can show a "send a message" snippet for.
struct Language {
    name: &'static str,
    /// How to add the Svix SDK to a project.
    install: &'static str,
    /// The snippet itself, rendered for the application and message created above.
    snippet: fn(app_id: &str, msg: &SampleMessage, server_url: &str) -> String,
}

const LANGUAGES: &[Language] = &[
    Language {
        name: "Python",
        install: "pip install svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"import os

from svix.api import Svix, MessageIn

svix = Svix(os.environ["SVIX_AUTH_TOKEN"])

# Call this wherever the event actually happens in your code.
svix.message.create(
    "{app_id}",
    MessageIn(event_type="{event_type}", payload={payload}),
)"#
            )
        },
    },
    Language {
        name: "JavaScript / TypeScript",
        install: "npm install svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"import {{ Svix }} from "svix";

const svix = new Svix(process.env.SVIX_AUTH_TOKEN);

// Call this wherever the event actually happens in your code.
await svix.message.create("{app_id}", {{
  eventType: "{event_type}",
  payload: {payload},
}});"#
            )
        },
    },
    Language {
        name: "Go",
        install: "go get github.com/svix/svix-webhooks/go",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"svixClient, err := svix.New(os.Getenv("SVIX_AUTH_TOKEN"), nil)
if err != nil {{
    return err
}}

// Call this wherever the event actually happens in your code.
var payload map[string]any
json.Unmarshal([]byte(`{payload}`), &payload)

_, err = svixClient.Message.Create(ctx, "{app_id}", models.MessageIn{{
    EventType: "{event_type}",
    Payload:   payload,
}}, nil)"#
            )
        },
    },
    Language {
        name: "Rust",
        install: "cargo add svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"let svix = Svix::new(std::env::var("SVIX_AUTH_TOKEN")?, None);

// Call this wherever the event actually happens in your code.
svix.message()
    .create(
        "{app_id}".to_owned(),
        MessageIn::new(
            "{event_type}".to_owned(),
            serde_json::json!({payload}),
        ),
        None,
    )
    .await?;"#
            )
        },
    },
    Language {
        name: "cURL (any language)",
        install: "no SDK needed",
        snippet: |app_id, msg, server_url| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"curl -X POST "{server_url}/api/v1/app/{app_id}/msg" \
  -H "Authorization: Bearer $SVIX_AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{{
    "eventType": "{event_type}",
    "payload": {payload}
  }}'"#
            )
        },
    },
];

/// The public API URL to show in the curl snippet when the config doesn't override it.
const DEFAULT_SERVER_URL: &str = "https://api.svix.com";

/// Step 4 (manual): show where and what to change in the user's own code.
fn integrate_code(cfg: &Config, app: &ApplicationOut, msg: &SampleMessage) -> anyhow::Result<()> {
    println!("Step 4: Add Svix to your code");

    let selection = dialoguer::Select::new()
        .with_prompt("Which language is your app written in?")
        .items(LANGUAGES.iter().map(|l| l.name).collect::<Vec<_>>())
        .default(0)
        .interact()?;
    let language = &LANGUAGES[selection];

    let server_url = cfg.server_url().unwrap_or(DEFAULT_SERVER_URL);

    println!("\n1. Install the SDK:\n");
    println!("   {}", green(language.install));
    println!("\n2. Keep your API token out of source control. Export it where your app runs:\n");
    println!("   {}", green("export SVIX_AUTH_TOKEN='<your-token>'"));
    println!(
        "\n   You can create environment-specific tokens in the dashboard: \
         https://dashboard.svix.com"
    );
    println!(
        "\n3. Send a message from the place in your code where the event happens — the same \
         \n   spot you'd log it or fire an internal event:\n"
    );

    for line in (language.snippet)(&app.id, msg, server_url).lines() {
        println!("   {line}");
    }

    println!(
        "\nIn your real integration you'd create one application per customer and use \
         \nthat customer's application ID (or uid) here, instead of the hardcoded one above.\n"
    );

    // Not a gate on anything, just a beat so the snippet doesn't scroll past.
    let _ = dialoguer::Confirm::new()
        .with_prompt("Ready to continue?")
        .default(true)
        .interact()?;
    println!();

    Ok(())
}
