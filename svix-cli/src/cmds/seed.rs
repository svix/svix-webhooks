use anyhow::Context;
use clap::Args;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use svix::api::*;

#[derive(Args)]
struct SeedOptions {
    /// Will clear out all the applications and event types
    #[arg(long, default_value = "false")]
    pub reset: bool,

    /// The number of endpoints to create (0-10)
    #[arg(long, value_parser = clap::value_parser!(u8).range(..=10) , default_value = "2")]
    pub endpoint_count: u8,

    /// The number of messages to create (0-10)
    #[arg(long, value_parser = clap::value_parser!(u8).range(..=100) , default_value = "10")]
    pub message_count: u8,
}

#[derive(Args)]
pub struct SeedArgs {
    #[clap(flatten)]
    options: SeedOptions,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct SeedOut {
    application: ApplicationOut,
    endpoints: Vec<String>,
    event_types: Vec<String>,
    messages: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayTokenOut {
    token: String,
}

const PLAY_TOKEN_GENERATE_URL: &str = "https://api.play.svix.com/api/v1/token/generate/";
const USER_EVENT_TYPES: [&str; 4] = ["signup", "signin", "signout", "deleted"];

pub async fn exec(
    client: &Svix,
    args: SeedArgs,
    color_mode: colored_json::ColorMode,
) -> anyhow::Result<()> {
    let mut seed_out = SeedOut {
        ..Default::default()
    };

    if args.options.reset {
        let confirmation = dialoguer::Confirm::new()
         .with_prompt("This will clear out all the applications and event types! Do you want to continue? ")
         .interact()
         .unwrap_or(false);

        if confirmation {
            reset_application(client).await?;
            reset_event_type(client).await?;
        } else {
            return Ok(());
        }
    }

    let application_in = ApplicationIn {
        name: format!("Test application"),
        ..Default::default()
    };
    let application_out = client.application().create(application_in, None).await?;

    seed_out.application = application_out.clone();

    let app_id = application_out.id;

    let mut handles = Vec::new();

    for _ in 0..args.options.endpoint_count {
        let client = client.clone();
        let app_id = app_id.clone();

        handles.push(tokio::spawn(async move {
            create_endpoint(client, app_id).await
        }))
    }

    for h in handles {
        let eo = h.await??;
        seed_out.endpoints.push(eo.url);
    }

    for uet in USER_EVENT_TYPES {
        let event_type_in = EventTypeIn {
            name: format!("user.{}", uet),
            description: "".to_string(),
            ..Default::default()
        };
        let Ok(event_type_out) = client.event_type().create(event_type_in, None).await else {
            continue;
        };
        seed_out.event_types.push(event_type_out.name);
    }
    let mut handles = Vec::new();

    for _ in 0..args.options.message_count {
        let client = client.clone();
        let app_id = app_id.clone();

        handles.push(tokio::spawn(
            async move { create_message(client, app_id).await },
        ))
    }

    for h in handles {
        let message_out = h.await??;
        seed_out.messages.push(message_out.id);
    }

    let summary = format!(
        "Seeded {} endpoints, {} event types, {} messages to application \"{}\"",
        seed_out.endpoints.len(),
        seed_out.event_types.len(),
        seed_out.messages.len(),
        seed_out.application.name
    );

    crate::json::print_json_output(&seed_out, color_mode)?;
    println!("{summary}");

    Ok(())
}

async fn create_endpoint(client: Svix, app_id: String) -> anyhow::Result<EndpointOut> {
    let req_client = reqwest::Client::new();

    let resp = req_client
        .post(PLAY_TOKEN_GENERATE_URL)
        .send()
        .await?
        .json::<PlayTokenOut>()
        .await
        .context("Failed to get token from public api")?;

    let endpoint_in = EndpointIn {
        url: format!("https://play.svix.com/in/{}/", resp.token),
        ..Default::default()
    };
    let endpoint_out = client.endpoint().create(app_id, endpoint_in, None).await?;
    Ok(endpoint_out)
}

async fn create_message(client: Svix, app_id: String) -> anyhow::Result<MessageOut> {
    let mut rng = StdRng::from_entropy();

    let event_type = USER_EVENT_TYPES
        .choose(&mut rng)
        .context("Couldn't pick a random event type while creating a message")?;

    let message_in = MessageIn {
        event_type: event_type.to_string(),
        payload: json!({}),
        ..Default::default()
    };

    let message_out = client.message().create(app_id, message_in, None).await?;
    Ok(message_out)
}

async fn reset_application(client: &Svix) -> anyhow::Result<()> {
    let resp = client.application().list(None).await?;
    let mut handles = Vec::new();

    for app_out in resp.data {
        let client = client.clone();

        handles.push(tokio::spawn(async move {
            if let Err(err) = client.application().delete(app_out.id.clone()).await {
                eprintln!("Failed to delete application {}: {}", app_out.id, err);
            }
        }));
    }

    for h in handles {
        let _ = h.await;
    }

    Ok(())
}

async fn reset_event_type(client: &Svix) -> anyhow::Result<()> {
    let resp = client.event_type().list(None).await?;
    let mut handles = Vec::new();

    for event_type_out in resp.data {
        let client = client.clone();

        let handle = tokio::spawn(async move {
            let _ = client
                .event_type()
                .delete(
                    event_type_out.name,
                    Some(EventTypeDeleteOptions {
                        expunge: Some(true),
                    }),
                )
                .await;
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }

    Ok(())
}
