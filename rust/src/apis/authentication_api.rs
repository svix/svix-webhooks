/*
 * Svix API
 *
 * Welcome to the Svix API documentation!  Useful links: [Homepage](https://www.svix.com) | [Support email](mailto:support+docs@svix.com) | [Blog](https://www.svix.com/blog/) | [Slack Community](https://www.svix.com/slack/)  # Introduction  This is the reference documentation and schemas for the [Svix webhook service](https://www.svix.com) API. For tutorials and other documentation please refer to [the documentation](https://docs.svix.com).  ## Main concepts  In Svix you have four important entities you will be interacting with:  - `messages`: these are the webhooks being sent. They can have contents and a few other properties. - `application`: this is where `messages` are sent to. Usually you want to create one application for each user on your platform. - `endpoint`: endpoints are the URLs messages will be sent to. Each application can have multiple `endpoints` and each message sent to that application will be sent to all of them (unless they are not subscribed to the sent event type). - `event-type`: event types are identifiers denoting the type of the message being sent. Event types are primarily used to decide which events are sent to which endpoint.   ## Authentication  Get your authentication token (`AUTH_TOKEN`) from the [Svix dashboard](https://dashboard.svix.com) and use it as part of the `Authorization` header as such: `Authorization: Bearer ${AUTH_TOKEN}`.  <SecurityDefinitions />   ## Code samples  The code samples assume you already have the respective libraries installed and you know how to use them. For the latest information on how to do that, please refer to [the documentation](https://docs.svix.com/).   ## Idempotency  Svix supports [idempotency](https://en.wikipedia.org/wiki/Idempotence) for safely retrying requests without accidentally performing the same operation twice. This is useful when an API call is disrupted in transit and you do not receive a response.  To perform an idempotent request, pass the idempotency key in the `Idempotency-Key` header to the request. The idempotency key should be a unique value generated by the client. You can create the key in however way you like, though we suggest using UUID v4, or any other string with enough entropy to avoid collisions.  Svix's idempotency works by saving the resulting status code and body of the first request made for any given idempotency key for any successful request. Subsequent requests with the same key return the same result.  Please note that idempotency is only supported for `POST` requests.   ## Cross-Origin Resource Sharing  This API features Cross-Origin Resource Sharing (CORS) implemented in compliance with [W3C spec](https://www.w3.org/TR/cors/). And that allows cross-domain communication from the browser. All responses have a wildcard same-origin which makes them completely public and accessible to everyone, including any code on any site. 
 *
 * The version of the OpenAPI document: 1.4
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `get_dashboard_access_api_v1_auth_dashboard_access_app_id_post`
#[derive(Clone, Debug)]
pub struct GetDashboardAccessApiV1AuthDashboardAccessAppIdPostParams {
    pub app_id: String,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method `logout_api_v1_auth_logout_post`
#[derive(Clone, Debug)]
pub struct LogoutApiV1AuthLogoutPostParams {
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}


/// struct for typed errors of method `get_dashboard_access_api_v1_auth_dashboard_access_app_id_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardAccessApiV1AuthDashboardAccessAppIdPostError {
    Status401(crate::models::HttpErrorOut),
    Status403(crate::models::HttpErrorOut),
    Status404(crate::models::HttpErrorOut),
    Status409(crate::models::HttpErrorOut),
    Status422(crate::models::HttpValidationError),
    Status429(crate::models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `logout_api_v1_auth_logout_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoutApiV1AuthLogoutPostError {
    Status401(crate::models::HttpErrorOut),
    Status403(crate::models::HttpErrorOut),
    Status404(crate::models::HttpErrorOut),
    Status409(crate::models::HttpErrorOut),
    Status422(crate::models::HttpValidationError),
    Status429(crate::models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}


/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
pub async fn get_dashboard_access_api_v1_auth_dashboard_access_app_id_post(configuration: &configuration::Configuration, params: GetDashboardAccessApiV1AuthDashboardAccessAppIdPostParams) -> Result<crate::models::DashboardAccessOut, Error<GetDashboardAccessApiV1AuthDashboardAccessAppIdPostError>> {
    // unbox the parameters
    let app_id = params.app_id;
    let idempotency_key = params.idempotency_key;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/auth/dashboard-access/{app_id}/", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = idempotency_key {
        local_var_req_builder = local_var_req_builder.header("idempotency-key", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDashboardAccessApiV1AuthDashboardAccessAppIdPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Logout an app token.  Trying to log out other tokens will fail.
pub async fn logout_api_v1_auth_logout_post(configuration: &configuration::Configuration, params: LogoutApiV1AuthLogoutPostParams) -> Result<(), Error<LogoutApiV1AuthLogoutPostError>> {
    // unbox the parameters
    let idempotency_key = params.idempotency_key;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/auth/logout/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = idempotency_key {
        local_var_req_builder = local_var_req_builder.header("idempotency-key", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LogoutApiV1AuthLogoutPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

