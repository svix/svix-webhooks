mod db;
mod e2e_application;
mod e2e_attempt;
mod e2e_auth;
mod e2e_endpoint;
mod e2e_event_type;
mod e2e_health;
mod e2e_message;
mod e2e_operational_webhooks;
mod e2e_proxy;
mod integ_webhook_http_client;
mod message_app;
mod redis_queue;
mod utils;
mod worker;

#[ctor::ctor]
fn test_setup() {
    svix_server::setup_tracing_for_tests();
}
