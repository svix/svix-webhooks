mod highlight;
mod tui;

use std::collections::BTreeSet;

use anyhow::Context as _;
use clap::{Args, Subcommand};
use svix::{
    api::Svix,
    models::{
        AppPortalAccessIn, ApplicationIn, ApplicationOut, EndpointIn, EndpointOut, EventTypeIn,
        MessageIn, MessageOut,
    },
};

use self::highlight::Syntax;

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
    // Anything the user will want to run or copy is printed after the UI's alternate
    // screen is gone.
    match tui::run().await? {
        Outcome::Manual => {}
        Outcome::SkillsInstalled => {
            println!("\nSvix agent skills installed.");
            print_agent_prompt();
        }
        Outcome::InstallByHand(scope) => {
            println!("\nInstall the Svix skills by running:\n");
            for skill in SKILL_NAMES {
                println!("\x1b[32m{}\x1b[0m", install_command(skill, scope).join(" "));
            }
            print_agent_prompt();
        }
    }

    Ok(())
}

/// How the user wants to work through the rest of the quickstart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuickstartMode {
    Manual,
    Agent,
}

/// What the wizard's run amounted to, and so what still gets printed on the way out.
#[derive(Clone, Copy)]
enum Outcome {
    /// The manual path, or quitting part way through the agent one.
    Manual,
    SkillsInstalled,
    InstallByHand(SkillScope),
}

/// The skills published at <https://github.com/svix/ai>, installed via the `skills` CLI.
const SKILLS_PACKAGE: &str = "svix/ai";

/// Installed with one command each: the `skills` CLI takes one skill at a time.
const SKILL_NAMES: &[&str] = &["svix-sending-webhooks", "receiving-webhooks"];

/// A starting point, not an incantation: the skills trigger on anything Svix-shaped.
const AGENT_PROMPT: &str = "Use Svix to start sending webhooks";

/// Where the skills get installed: alongside the current project, or user-wide.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SkillScope {
    Project,
    Global,
}

impl SkillScope {
    fn flag(self) -> &'static str {
        match self {
            Self::Project => "--project",
            Self::Global => "--global",
        }
    }
}

fn print_agent_prompt() {
    println!("\nThen ask your coding agent something like:\n");
    // Green, like the login flow's verification code.
    println!("\x1b[32m{AGENT_PROMPT}\x1b[0m\n");
    println!("The skills take it from there.");
}

/// The argv that installs one skill. Shown to the user verbatim, so it has to be
/// runnable by hand; `-y` skips the `skills` CLI prompts.
fn install_command(skill: &'static str, scope: SkillScope) -> Vec<&'static str> {
    vec![
        "npx",
        "--yes",
        "skills",
        "add",
        SKILLS_PACKAGE,
        "-y",
        scope.flag(),
        "--skill",
        skill,
    ]
}

/// Installs a single skill. Output is captured, not inherited: the wizard still owns
/// the screen, and the `skills` CLI draws an installer of its own.
fn install_skill(skill: &'static str, scope: SkillScope) -> anyhow::Result<()> {
    let command = install_command(skill, scope);

    let output = std::process::Command::new(command[0])
        .args(&command[1..])
        .output()
        .context(
            "Failed to run `npx`. Install Node.js and try again, or install the skills \
             yourself with `npx skills add svix/ai`.",
        )?;

    if !output.status.success() {
        anyhow::bail!(
            "`{}` failed:\n{}",
            command.join(" "),
            last_lines(&output.stderr, 5)
        );
    }

    Ok(())
}

/// The last non-empty lines of a stream: the failure's reason without the whole log.
fn last_lines(stream: &[u8], count: usize) -> String {
    let text = String::from_utf8_lossy(stream);
    let lines: Vec<_> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let start = lines.len().saturating_sub(count);

    lines[start..].join("\n")
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

const APP_NAME: &str = "My first app";
/// Gets a random suffix so repeated runs don't collide.
const APP_UID_PREFIX: &str = "quickstart-";

/// The application the quickstart works against, plus the Svix Play inbox it delivers to.
struct Quickstart {
    app: ApplicationOut,
    /// The example endpoint pointing at the Svix Play inbox.
    endpoint: EndpointOut,
    /// Where the delivered message can be inspected in the browser.
    play_view_url: String,
    /// Set when the org requires endpoints to specify channels; the message has to be
    /// sent on it to reach the endpoint.
    channel: Option<String>,
}

/// The channel used when the org requires one, same as the dashboard onboarding.
const CHANNEL: &str = "my-channel";

/// Creates the application plus a Svix Play endpoint, mirroring the dashboard
/// onboarding: the first message gets a destination without the user running a server.
async fn create_application(client: &Svix, msg: &SampleMessage) -> anyhow::Result<Quickstart> {
    let application_in = ApplicationIn {
        uid: Some(format!("{APP_UID_PREFIX}{}", generate_token(8))),
        ..ApplicationIn::new(APP_NAME.to_owned())
    };
    let app = client
        .application()
        .create(application_in, None)
        .await
        .context("Failed to create the application")?;

    let play_token = generate_token(PLAY_TOKEN_LEN);
    let (endpoint, channel) = create_play_endpoint(client, &app.id, &play_token, msg).await?;

    Ok(Quickstart {
        app,
        endpoint,
        play_view_url: format!("{PLAY_URL}/view/{play_token}/"),
        channel,
    })
}

/// Creates the Svix Play endpoint, returning it and the channel it listens on (if any).
///
/// Orgs can require endpoints to specify filter types and/or channels. Filter types are
/// always sent; the channel only after the API rejects the first attempt for lacking
/// one, since those org settings aren't exposed through the public API.
async fn create_play_endpoint(
    client: &Svix,
    app_id: &str,
    play_token: &str,
    msg: &SampleMessage,
) -> anyhow::Result<(EndpointOut, Option<String>)> {
    let endpoint_in = EndpointIn {
        description: Some("Svix onboarding endpoint".to_owned()),
        // Serialized as `filterTypes`.
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

/// Fallback matching the dashboard onboarding when the account has no event types yet.
const DEFAULT_EVENT_TYPE: &str = "invoice.paid";

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
            let event_type_in = EventTypeIn::new(
                DEFAULT_EVENT_TYPE.to_owned(),
                "An invoice was paid".to_owned(),
            );
            client
                .event_type()
                .create(event_type_in, None)
                .await
                .with_context(|| {
                    format!("Failed to create the `{DEFAULT_EVENT_TYPE}` event type")
                })?;

            DEFAULT_EVENT_TYPE.to_owned()
        }
    };

    Ok(SampleMessage {
        event_type,
        payload: serde_json::json!({ "id": "invoice_WF7WtC", "status": "paid", "attempt": 1 }),
    })
}

/// A language the quickstart can show its code samples in.
struct Language {
    name: &'static str,
    /// How to add the Svix SDK to a project.
    install: &'static str,
    /// Sends the sample message.
    snippet: fn(app_id: &str, msg: &SampleMessage, server_url: &str) -> String,
    /// Mints an app portal magic link from the user's own backend.
    portal: fn(app_id: &str, server_url: &str) -> String,
    syntax: Syntax,
}

const LANGUAGES: &[Language] = &[
    Language {
        name: "Python",
        syntax: Syntax::Python,
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
        portal: |app_id, _| {
            format!(
                r#"from svix.api import Svix, AppPortalAccessIn

svix = Svix(os.environ["SVIX_AUTH_TOKEN"])

# Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
access = svix.authentication.app_portal_access("{app_id}", AppPortalAccessIn())
portal_url = access.url"#
            )
        },
    },
    Language {
        name: "JavaScript / TypeScript",
        syntax: Syntax::JavaScript,
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
        portal: |app_id, _| {
            format!(
                r#"import {{ Svix }} from "svix";

const svix = new Svix(process.env.SVIX_AUTH_TOKEN);

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
const access = await svix.authentication.appPortalAccess("{app_id}", {{}});
const portalUrl = access.url;"#
            )
        },
    },
    Language {
        name: "Go",
        syntax: Syntax::Go,
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
        portal: |app_id, _| {
            format!(
                r#"svixClient, err := svix.New(os.Getenv("SVIX_AUTH_TOKEN"), nil)
if err != nil {{
    return err
}}

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
access, err := svixClient.Authentication.AppPortalAccess(
    ctx, "{app_id}", models.AppPortalAccessIn{{}}, nil,
)
if err != nil {{
    return err
}}
portalURL := access.Url"#
            )
        },
    },
    Language {
        name: "Rust",
        syntax: Syntax::Rust,
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
        portal: |app_id, _| {
            format!(
                r#"let svix = Svix::new(std::env::var("SVIX_AUTH_TOKEN")?, None);

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
let access = svix
    .authentication()
    .app_portal_access("{app_id}".to_owned(), AppPortalAccessIn::new(), None)
    .await?;
let portal_url = access.url;"#
            )
        },
    },
    Language {
        name: "Java",
        syntax: Syntax::Java,
        install: "# add \"com.svix:svix\" to your Gradle or Maven dependencies",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"import com.svix.Svix;
import com.svix.models.MessageIn;

Svix svix = new Svix(System.getenv("SVIX_AUTH_TOKEN"));

// Call this wherever the event actually happens in your code.
svix.getMessage().create(
    "{app_id}",
    new MessageIn()
        .eventType("{event_type}")
        .payload("""
            {payload}"""));"#
            )
        },
        portal: |app_id, _| {
            format!(
                r#"import com.svix.Svix;
import com.svix.models.AppPortalAccessIn;

Svix svix = new Svix(System.getenv("SVIX_AUTH_TOKEN"));

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
var access = svix.getAuthentication()
    .appPortalAccess("{app_id}", new AppPortalAccessIn());
var portalUrl = access.getUrl();"#
            )
        },
    },
    Language {
        name: "Kotlin",
        syntax: Syntax::Kotlin,
        install: "# add \"com.svix.kotlin:svix-kotlin\" to your Gradle dependencies",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"import com.svix.kotlin.Svix
import com.svix.kotlin.models.MessageIn

val svix = Svix(System.getenv("SVIX_AUTH_TOKEN"))

// Call this wherever the event actually happens in your code.
svix.message.create(
    "{app_id}",
    MessageIn(eventType = "{event_type}", payload = """{payload}"""),
)"#
            )
        },
        portal: |app_id, _| {
            format!(
                r#"import com.svix.kotlin.Svix
import com.svix.kotlin.models.AppPortalAccessIn

val svix = Svix(System.getenv("SVIX_AUTH_TOKEN"))

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
val access = svix.authentication.appPortalAccess("{app_id}", AppPortalAccessIn())
val portalUrl = access.url"#
            )
        },
    },
    Language {
        name: "C#",
        syntax: Syntax::CSharp,
        install: "dotnet add package Svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"using Newtonsoft.Json.Linq;
using Svix;
using Svix.Models;

var svix = new SvixClient(Environment.GetEnvironmentVariable("SVIX_AUTH_TOKEN")!);

// Call this wherever the event actually happens in your code.
await svix.Message.CreateAsync(
    "{app_id}",
    new MessageIn
    {{
        EventType = "{event_type}",
        Payload = JObject.Parse("""{payload}"""),
    }}
);"#
            )
        },
        portal: |app_id, _| {
            format!(
                r#"using Svix;
using Svix.Models;

var svix = new SvixClient(Environment.GetEnvironmentVariable("SVIX_AUTH_TOKEN")!);

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
var access = await svix.Authentication.AppPortalAccessAsync(
    "{app_id}", new AppPortalAccessIn());
var portalUrl = access.Url;"#
            )
        },
    },
    Language {
        name: "Ruby",
        syntax: Syntax::Ruby,
        install: "gem install svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"require "svix"

svix = Svix::Client.new(ENV["SVIX_AUTH_TOKEN"])

# Call this wherever the event actually happens in your code.
svix.message.create(
  "{app_id}",
  Svix::MessageIn.new(event_type: "{event_type}", payload: {payload})
)"#
            )
        },
        portal: |app_id, _| {
            format!(
                r#"require "svix"

svix = Svix::Client.new(ENV["SVIX_AUTH_TOKEN"])

# Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
access = svix.authentication.app_portal_access("{app_id}", Svix::AppPortalAccessIn.new)
portal_url = access.url"#
            )
        },
    },
    Language {
        name: "PHP",
        syntax: Syntax::Php,
        install: "composer require svix/svix",
        snippet: |app_id, msg, _| {
            let SampleMessage {
                event_type,
                payload,
            } = msg;
            format!(
                r#"<?php
$svix = new \Svix\Svix(getenv("SVIX_AUTH_TOKEN"));

// Call this wherever the event actually happens in your code.
$svix->message->create(
    "{app_id}",
    \Svix\Models\MessageIn::create("{event_type}", json_decode('{payload}', true))
);"#
            )
        },
        portal: |app_id, _| {
            format!(
                r#"<?php
$svix = new \Svix\Svix(getenv("SVIX_AUTH_TOKEN"));

// Serve this URL from your own dashboard, e.g. behind a "Webhooks" button.
$access = $svix->authentication->appPortalAccess(
    "{app_id}",
    \Svix\Models\AppPortalAccessIn::create()
);
$portalUrl = $access->url;"#
            )
        },
    },
    Language {
        name: "cURL (any language)",
        syntax: Syntax::Shell,
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
        portal: |app_id, server_url| {
            format!(
                r#"# Serve the returned `url` from your own dashboard, e.g. behind a "Webhooks" button.
curl -X POST "{server_url}/api/v1/auth/app-portal-access/{app_id}" \
  -H "Authorization: Bearer $SVIX_AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{{}}'"#
            )
        },
    },
];

/// Shown in the curl snippet when the config doesn't override it.
const DEFAULT_SERVER_URL: &str = "https://api.svix.com";

/// Sends the sample message to the application's endpoints.
async fn send_message(
    client: &Svix,
    qs: &Quickstart,
    msg: &SampleMessage,
) -> anyhow::Result<MessageOut> {
    let message_in = MessageIn {
        // The endpoint only listens on this channel when the org requires one.
        channels: qs
            .channel
            .as_ref()
            .map(|channel| BTreeSet::from([channel.clone()])),
        ..MessageIn::new(msg.event_type.clone(), msg.payload.clone())
    };
    client
        .message()
        .create(qs.app.id.clone(), message_in, None)
        .await
        .context("Failed to send the message")
}

/// Mints a magic link into the app portal for the quickstart's application.
async fn portal_url(client: &Svix, qs: &Quickstart) -> anyhow::Result<String> {
    let access = client
        .authentication()
        .app_portal_access(qs.app.id.clone(), AppPortalAccessIn::new(), None)
        .await
        .context("Failed to generate an app portal URL")?;

    // `dashboardTour` turns on the guided tour, like the dashboard onboarding link.
    Ok(with_dashboard_tour(&access.url))
}

/// Adds the `dashboardTour` query param to an app portal magic link.
///
/// The one-time key lives in the URL fragment (`.../login#key=...`), so the param has to
/// go before the `#` for the app portal to see it.
fn with_dashboard_tour(url: &str) -> String {
    let (base, fragment) = match url.split_once('#') {
        Some((base, fragment)) => (base, Some(fragment)),
        None => (url, None),
    };
    let separator = if base.contains('?') { '&' } else { '?' };

    match fragment {
        Some(fragment) => format!("{base}{separator}dashboardTour=true#{fragment}"),
        None => format!("{base}{separator}dashboardTour=true"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_renders_its_samples_for_the_app() {
        let msg = SampleMessage {
            event_type: "invoice.paid".to_owned(),
            payload: serde_json::json!({ "id": "invoice_WF7WtC" }),
        };

        for language in LANGUAGES {
            let snippet = (language.snippet)("app_123", &msg, DEFAULT_SERVER_URL);
            assert!(snippet.contains("app_123"), "{}: {snippet}", language.name);
            assert!(
                snippet.contains("invoice.paid"),
                "{}: {snippet}",
                language.name
            );
            assert!(
                snippet.contains("invoice_WF7WtC"),
                "{}: {snippet}",
                language.name
            );

            let portal = (language.portal)("app_123", DEFAULT_SERVER_URL);
            assert!(portal.contains("app_123"), "{}: {portal}", language.name);
        }
    }

    #[test]
    fn dashboard_tour_param_goes_before_the_fragment() {
        assert_eq!(
            with_dashboard_tour("https://app.svix.com/login#key=abc"),
            "https://app.svix.com/login?dashboardTour=true#key=abc"
        );
        assert_eq!(
            with_dashboard_tour("https://app.svix.com/login?foo=1#key=abc"),
            "https://app.svix.com/login?foo=1&dashboardTour=true#key=abc"
        );
        assert_eq!(
            with_dashboard_tour("https://app.svix.com/login"),
            "https://app.svix.com/login?dashboardTour=true"
        );
    }
}
