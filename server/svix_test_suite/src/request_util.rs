use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApplicationOut {
    pub uid: Option<String>,
    pub name: String,
    pub rate_limit: Option<u16>,

    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) async fn create_application(
    base_uri: &str,
    token: &str,
    application_name: &str,
) -> Result<(StatusCode, Result<ApplicationOut>)> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/api/v1/app/", base_uri))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "name": application_name }))
        .send()
        .await
        .context(format!(
            "creating application at base_uri {} with token {} and name {}",
            base_uri,
            token,
            application_name
        ))?;

    Ok((
        response.status(),
        response
            .json::<ApplicationOut>()
            .await
            .context("error deserializing to ApplicationOut"),
    ))
}

pub(crate) async fn read_application(
    base_uri: &str,
    token: &str,
    application_id: &str,
) -> Result<(StatusCode, Result<ApplicationOut>)> {
    let client = Client::new();
    let response = client
        .get(&format!("{}/api/v1/app/{}/", base_uri, application_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context(format!(
            "creating application at base_uri {} with token {} and id {}",
            base_uri,
            token,
            application_id
        ))?;

    Ok((
        response.status(),
        response
            .json::<ApplicationOut>()
            .await
            .context("error deserializing to ApplicationOut"),
    ))
}

pub(crate) async fn delete_application(
    base_uri: &str,
    token: &str,
    application_id: &str,
) -> Result<(StatusCode, Result<()>)> {
    let client = Client::new();
    let response = client
        .delete(&format!("{}/api/v1/app/{}/", base_uri, application_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context(format!(
            "creating application at base_uri {} with token {} and id {}",
            base_uri,
            token,
            application_id
        ))?;

    Ok((
        response.status(),
        response
            .json::<()>()
            .await
            .context("error deserializing to ()"),
    ))
}
