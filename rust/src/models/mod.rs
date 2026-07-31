// this file is @generated
#![allow(clippy::too_many_arguments)]

mod adobe_sign_config;
mod adobe_sign_config_out;
mod aggregate_event_types_out;
mod airwallex_config;
mod airwallex_config_out;
mod amazon_s3_patch_config;
mod api_token_out;
mod app_portal_access_in;
mod app_portal_access_out;
mod app_portal_capability;
mod app_usage_stats_in;
mod app_usage_stats_out;
mod application_in;
mod application_out;
mod application_patch;
mod application_token_expire_in;
mod authentication_source;
mod auto_config_sink_type;
mod azure_blob_storage_config;
mod azure_blob_storage_patch_config;
mod background_task_out;
mod background_task_status;
mod background_task_type;
mod big_query_config;
mod big_query_patch_config;
mod bulk_replay_in;
mod checkbook_config;
mod checkbook_config_out;
mod clickhouse_config;
mod clickhouse_patch_config;
mod connector_in;
mod connector_kind;
mod connector_out;
mod connector_patch;
mod connector_product;
mod connector_upsert_in;
mod create_stream_events_in;
mod create_stream_events_out;
mod cron_config;
mod docusign_config;
mod docusign_config_out;
mod easypost_config;
mod easypost_config_out;
mod empty_response;
mod endpoint_headers_in;
mod endpoint_headers_out;
mod endpoint_headers_patch_in;
mod endpoint_in;
mod endpoint_message_out;
mod endpoint_out;
mod endpoint_patch;
mod endpoint_secret_out;
mod endpoint_secret_rotate_in;
mod endpoint_stats;
mod endpoint_transformation_in;
mod endpoint_transformation_out;
mod endpoint_transformation_patch;
mod endpoint_upsert_in;
mod environment_in;
mod environment_out;
mod event_bridge_config;
mod event_bridge_patch_config;
mod event_example_in;
mod event_in;
mod event_out;
mod event_stream_out;
mod event_type_from_open_api;
mod event_type_import_open_api_in;
mod event_type_import_open_api_out;
mod event_type_import_open_api_out_data;
mod event_type_in;
mod event_type_out;
mod event_type_patch;
mod event_type_upsert_in;
mod expunge_all_contents_out;
mod github_config;
mod github_config_out;
mod google_cloud_pub_sub_config;
mod google_cloud_pub_sub_patch_config;
mod google_cloud_storage_config;
mod google_cloud_storage_patch_config;
mod http_patch_config;
mod http_sink_headers_patch_in;
mod hubspot_config;
mod hubspot_config_out;
mod ingest_endpoint_headers_in;
mod ingest_endpoint_headers_out;
mod ingest_endpoint_in;
mod ingest_endpoint_out;
mod ingest_endpoint_secret_in;
mod ingest_endpoint_secret_out;
mod ingest_endpoint_transformation_out;
mod ingest_endpoint_transformation_patch;
mod ingest_endpoint_upsert_in;
mod ingest_source_consumer_portal_access_in;
mod ingest_source_in;
mod ingest_source_out;
mod integration_in;
mod integration_key_out;
mod integration_out;
mod integration_update;
mod list_response_application_out;
mod list_response_background_task_out;
mod list_response_connector_out;
mod list_response_endpoint_message_out;
mod list_response_endpoint_out;
mod list_response_event_type_out;
mod list_response_ingest_endpoint_out;
mod list_response_ingest_source_out;
mod list_response_integration_out;
mod list_response_message_attempt_out;
mod list_response_message_endpoint_out;
mod list_response_message_out;
mod list_response_operational_webhook_endpoint_out;
mod list_response_stream_event_type_out;
mod list_response_stream_out;
mod list_response_stream_sink_out;
mod message_attempt_out;
mod message_attempt_trigger_type;
mod message_endpoint_out;
mod message_in;
mod message_out;
mod message_precheck_in;
mod message_precheck_out;
mod message_status;
mod message_status_text;
mod meta_config;
mod meta_config_out;
mod nango_config;
mod nango_config_out;
mod open_claw_config;
mod open_claw_config_out;
mod operational_webhook_endpoint_headers_in;
mod operational_webhook_endpoint_headers_out;
mod operational_webhook_endpoint_in;
mod operational_webhook_endpoint_out;
mod operational_webhook_endpoint_secret_in;
mod operational_webhook_endpoint_secret_out;
mod operational_webhook_endpoint_upsert_in;
mod ordering;
mod orum_io_config;
mod orum_io_config_out;
mod otel_tracing_patch_config;
mod panda_doc_config;
mod panda_doc_config_out;
mod poller_v2_commit_in;
mod poller_v2_message_out;
mod poller_v2_poll_out;
mod polling_endpoint_consumer_seek_in;
mod polling_endpoint_consumer_seek_out;
mod polling_endpoint_message_out;
mod polling_endpoint_out;
mod port_io_config;
mod port_io_config_out;
mod rabbit_mq_config;
mod rabbit_mq_patch_config;
mod recover_in;
mod recover_out;
mod redshift_config;
mod redshift_patch_config;
mod replay_in;
mod replay_out;
mod rotate_poller_token_in;
mod rotate_token_out;
mod rutter_config;
mod rutter_config_out;
mod s3_config;
mod segment_config;
mod segment_config_out;
mod shopify_config;
mod shopify_config_out;
mod sink_http_config;
mod sink_in_common;
mod sink_otel_v1_config;
mod sink_secret_out;
mod sink_status;
mod sink_status_in;
mod sink_transform_in;
mod sink_transformation_out;
mod slack_config;
mod slack_config_out;
mod snowflake_config;
mod snowflake_patch_config;
mod sns_config;
mod sns_patch_config;
mod sqs_config;
mod sqs_patch_config;
mod starting_position;
mod status_code_class;
mod stream_event_type_in;
mod stream_event_type_out;
mod stream_event_type_patch;
mod stream_in;
mod stream_out;
mod stream_patch;
mod stream_portal_access_in;
mod stream_sink_in;
mod stream_sink_out;
mod stream_sink_patch;
mod stream_token_expire_in;
mod stripe_config;
mod stripe_config_out;
mod subscribe_in;
mod svix_config;
mod svix_config_out;
mod tailscale_config;
mod tailscale_config_out;
mod telnyx_config;
mod telnyx_config_out;
mod vapi_config;
mod vapi_config_out;
mod veriff_config;
mod veriff_config_out;
mod vgs_config;
mod vgs_config_out;
mod whoami_out;
mod zoom_config;
mod zoom_config_out;
// not currently generated
mod http_error_out;
mod http_validation_error;
#[cfg(feature = "svix_beta")]
mod message_events_out;
mod validation_error;

pub use self::{
    adobe_sign_config::AdobeSignConfig,
    adobe_sign_config_out::AdobeSignConfigOut,
    aggregate_event_types_out::AggregateEventTypesOut,
    airwallex_config::AirwallexConfig,
    airwallex_config_out::AirwallexConfigOut,
    amazon_s3_patch_config::AmazonS3PatchConfig,
    api_token_out::ApiTokenOut,
    app_portal_access_in::AppPortalAccessIn,
    app_portal_access_out::AppPortalAccessOut,
    app_portal_capability::AppPortalCapability,
    app_usage_stats_in::AppUsageStatsIn,
    app_usage_stats_out::AppUsageStatsOut,
    application_in::ApplicationIn,
    application_out::ApplicationOut,
    application_patch::ApplicationPatch,
    application_token_expire_in::ApplicationTokenExpireIn,
    authentication_source::AuthenticationSource,
    auto_config_sink_type::AutoConfigSinkType,
    azure_blob_storage_config::AzureBlobStorageConfig,
    azure_blob_storage_patch_config::AzureBlobStoragePatchConfig,
    background_task_out::BackgroundTaskOut,
    background_task_status::BackgroundTaskStatus,
    background_task_type::BackgroundTaskType,
    big_query_config::BigQueryConfig,
    big_query_patch_config::BigQueryPatchConfig,
    bulk_replay_in::BulkReplayIn,
    checkbook_config::CheckbookConfig,
    checkbook_config_out::CheckbookConfigOut,
    clickhouse_config::ClickhouseConfig,
    clickhouse_patch_config::ClickhousePatchConfig,
    connector_in::ConnectorIn,
    connector_kind::ConnectorKind,
    connector_out::ConnectorOut,
    connector_patch::ConnectorPatch,
    connector_product::ConnectorProduct,
    connector_upsert_in::ConnectorUpsertIn,
    create_stream_events_in::CreateStreamEventsIn,
    create_stream_events_out::CreateStreamEventsOut,
    cron_config::CronConfig,
    docusign_config::DocusignConfig,
    docusign_config_out::DocusignConfigOut,
    easypost_config::EasypostConfig,
    easypost_config_out::EasypostConfigOut,
    empty_response::EmptyResponse,
    endpoint_headers_in::EndpointHeadersIn,
    endpoint_headers_out::EndpointHeadersOut,
    endpoint_headers_patch_in::EndpointHeadersPatchIn,
    endpoint_in::EndpointIn,
    endpoint_message_out::EndpointMessageOut,
    endpoint_out::EndpointOut,
    endpoint_patch::EndpointPatch,
    endpoint_secret_out::EndpointSecretOut,
    endpoint_secret_rotate_in::EndpointSecretRotateIn,
    endpoint_stats::EndpointStats,
    endpoint_transformation_in::EndpointTransformationIn,
    endpoint_transformation_out::EndpointTransformationOut,
    endpoint_transformation_patch::EndpointTransformationPatch,
    endpoint_upsert_in::EndpointUpsertIn,
    environment_in::EnvironmentIn,
    environment_out::EnvironmentOut,
    event_bridge_config::EventBridgeConfig,
    event_bridge_patch_config::EventBridgePatchConfig,
    event_example_in::EventExampleIn,
    event_in::EventIn,
    event_out::EventOut,
    event_stream_out::EventStreamOut,
    event_type_from_open_api::EventTypeFromOpenApi,
    event_type_import_open_api_in::EventTypeImportOpenApiIn,
    event_type_import_open_api_out::EventTypeImportOpenApiOut,
    event_type_import_open_api_out_data::EventTypeImportOpenApiOutData,
    event_type_in::EventTypeIn,
    event_type_out::EventTypeOut,
    event_type_patch::EventTypePatch,
    event_type_upsert_in::EventTypeUpsertIn,
    expunge_all_contents_out::ExpungeAllContentsOut,
    github_config::GithubConfig,
    github_config_out::GithubConfigOut,
    google_cloud_pub_sub_config::GoogleCloudPubSubConfig,
    google_cloud_pub_sub_patch_config::GoogleCloudPubSubPatchConfig,
    google_cloud_storage_config::GoogleCloudStorageConfig,
    google_cloud_storage_patch_config::GoogleCloudStoragePatchConfig,
    http_patch_config::HttpPatchConfig,
    http_sink_headers_patch_in::HttpSinkHeadersPatchIn,
    hubspot_config::HubspotConfig,
    hubspot_config_out::HubspotConfigOut,
    ingest_endpoint_headers_in::IngestEndpointHeadersIn,
    ingest_endpoint_headers_out::IngestEndpointHeadersOut,
    ingest_endpoint_in::IngestEndpointIn,
    ingest_endpoint_out::IngestEndpointOut,
    ingest_endpoint_secret_in::IngestEndpointSecretIn,
    ingest_endpoint_secret_out::IngestEndpointSecretOut,
    ingest_endpoint_transformation_out::IngestEndpointTransformationOut,
    ingest_endpoint_transformation_patch::IngestEndpointTransformationPatch,
    ingest_endpoint_upsert_in::IngestEndpointUpsertIn,
    ingest_source_consumer_portal_access_in::IngestSourceConsumerPortalAccessIn,
    ingest_source_in::{IngestSourceIn, IngestSourceInConfig},
    ingest_source_out::{IngestSourceOut, IngestSourceOutConfig},
    integration_in::IntegrationIn,
    integration_key_out::IntegrationKeyOut,
    integration_out::IntegrationOut,
    integration_update::IntegrationUpdate,
    list_response_application_out::ListResponseApplicationOut,
    list_response_background_task_out::ListResponseBackgroundTaskOut,
    list_response_connector_out::ListResponseConnectorOut,
    list_response_endpoint_message_out::ListResponseEndpointMessageOut,
    list_response_endpoint_out::ListResponseEndpointOut,
    list_response_event_type_out::ListResponseEventTypeOut,
    list_response_ingest_endpoint_out::ListResponseIngestEndpointOut,
    list_response_ingest_source_out::ListResponseIngestSourceOut,
    list_response_integration_out::ListResponseIntegrationOut,
    list_response_message_attempt_out::ListResponseMessageAttemptOut,
    list_response_message_endpoint_out::ListResponseMessageEndpointOut,
    list_response_message_out::ListResponseMessageOut,
    list_response_operational_webhook_endpoint_out::ListResponseOperationalWebhookEndpointOut,
    list_response_stream_event_type_out::ListResponseStreamEventTypeOut,
    list_response_stream_out::ListResponseStreamOut,
    list_response_stream_sink_out::ListResponseStreamSinkOut,
    message_attempt_out::MessageAttemptOut,
    message_attempt_trigger_type::MessageAttemptTriggerType,
    message_endpoint_out::MessageEndpointOut,
    message_in::MessageIn,
    message_out::MessageOut,
    message_precheck_in::MessagePrecheckIn,
    message_precheck_out::MessagePrecheckOut,
    message_status::MessageStatus,
    message_status_text::MessageStatusText,
    meta_config::MetaConfig,
    meta_config_out::MetaConfigOut,
    nango_config::NangoConfig,
    nango_config_out::NangoConfigOut,
    open_claw_config::OpenClawConfig,
    open_claw_config_out::OpenClawConfigOut,
    operational_webhook_endpoint_headers_in::OperationalWebhookEndpointHeadersIn,
    operational_webhook_endpoint_headers_out::OperationalWebhookEndpointHeadersOut,
    operational_webhook_endpoint_in::OperationalWebhookEndpointIn,
    operational_webhook_endpoint_out::OperationalWebhookEndpointOut,
    operational_webhook_endpoint_secret_in::OperationalWebhookEndpointSecretIn,
    operational_webhook_endpoint_secret_out::OperationalWebhookEndpointSecretOut,
    operational_webhook_endpoint_upsert_in::OperationalWebhookEndpointUpsertIn,
    ordering::Ordering,
    orum_io_config::OrumIoConfig,
    orum_io_config_out::OrumIoConfigOut,
    otel_tracing_patch_config::OtelTracingPatchConfig,
    panda_doc_config::PandaDocConfig,
    panda_doc_config_out::PandaDocConfigOut,
    poller_v2_commit_in::PollerV2CommitIn,
    poller_v2_message_out::PollerV2MessageOut,
    poller_v2_poll_out::PollerV2PollOut,
    polling_endpoint_consumer_seek_in::PollingEndpointConsumerSeekIn,
    polling_endpoint_consumer_seek_out::PollingEndpointConsumerSeekOut,
    polling_endpoint_message_out::PollingEndpointMessageOut,
    polling_endpoint_out::PollingEndpointOut,
    port_io_config::PortIoConfig,
    port_io_config_out::PortIoConfigOut,
    rabbit_mq_config::RabbitMqConfig,
    rabbit_mq_patch_config::RabbitMqPatchConfig,
    recover_in::RecoverIn,
    recover_out::RecoverOut,
    redshift_config::RedshiftConfig,
    redshift_patch_config::RedshiftPatchConfig,
    replay_in::ReplayIn,
    replay_out::ReplayOut,
    rotate_poller_token_in::RotatePollerTokenIn,
    rotate_token_out::RotateTokenOut,
    rutter_config::RutterConfig,
    rutter_config_out::RutterConfigOut,
    s3_config::S3Config,
    segment_config::SegmentConfig,
    segment_config_out::SegmentConfigOut,
    shopify_config::ShopifyConfig,
    shopify_config_out::ShopifyConfigOut,
    sink_http_config::SinkHttpConfig,
    sink_in_common::SinkInCommon,
    sink_otel_v1_config::SinkOtelV1Config,
    sink_secret_out::SinkSecretOut,
    sink_status::SinkStatus,
    sink_status_in::SinkStatusIn,
    sink_transform_in::SinkTransformIn,
    sink_transformation_out::SinkTransformationOut,
    slack_config::SlackConfig,
    slack_config_out::SlackConfigOut,
    snowflake_config::SnowflakeConfig,
    snowflake_patch_config::SnowflakePatchConfig,
    sns_config::SnsConfig,
    sns_patch_config::SnsPatchConfig,
    sqs_config::SqsConfig,
    sqs_patch_config::SqsPatchConfig,
    starting_position::StartingPosition,
    status_code_class::StatusCodeClass,
    stream_event_type_in::StreamEventTypeIn,
    stream_event_type_out::StreamEventTypeOut,
    stream_event_type_patch::StreamEventTypePatch,
    stream_in::StreamIn,
    stream_out::StreamOut,
    stream_patch::StreamPatch,
    stream_portal_access_in::StreamPortalAccessIn,
    stream_sink_in::{StreamSinkIn, StreamSinkInConfig},
    stream_sink_out::{StreamSinkOut, StreamSinkOutConfig},
    stream_sink_patch::{StreamSinkPatch, StreamSinkPatchConfig},
    stream_token_expire_in::StreamTokenExpireIn,
    stripe_config::StripeConfig,
    stripe_config_out::StripeConfigOut,
    subscribe_in::SubscribeIn,
    svix_config::SvixConfig,
    svix_config_out::SvixConfigOut,
    tailscale_config::TailscaleConfig,
    tailscale_config_out::TailscaleConfigOut,
    telnyx_config::TelnyxConfig,
    telnyx_config_out::TelnyxConfigOut,
    vapi_config::VapiConfig,
    vapi_config_out::VapiConfigOut,
    veriff_config::VeriffConfig,
    veriff_config_out::VeriffConfigOut,
    vgs_config::VgsConfig,
    vgs_config_out::VgsConfigOut,
    whoami_out::WhoamiOut,
    zoom_config::ZoomConfig,
    zoom_config_out::ZoomConfigOut,
};

// not currently generated
#[cfg(feature = "svix_beta")]
pub use self::message_events_out::MessageEventsOut;
pub use self::{
    http_error_out::HttpErrorOut, http_validation_error::HttpValidationError,
    validation_error::ValidationError,
};
