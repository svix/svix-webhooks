# OpenTelemetry Configuration
Svix supports sending tracing information to the OpenTelemetry collector at a configured address. This can be used to interface with many services such as DataDog or Sentry.

To set this up you can follow these steps:

1. Set up an account with any service which supports receiving logs from the [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/). Note some services (such as DataDog) require use of specific forks of the OpenTelemetry Collector which include "exporters" for their services. The most popular is the [opentelemetry-collector-contrib](https://github.com/open-telemetry/opentelemetry-collector-contrib).
2. Install, configure, and (re)start the OpenTelemetry Collector, noting the gRPC address it is configured to bind to. Ensure that your external service has been integrated properly by consulting their documentation for OpenTelemetry Collector exporters.
3. Configure the Svix server including the `opentelemetry_address` field to point towards the gRPC address configured above.
4. Optionally configure the `opentelemetry_sample_ration` in the Svix server configuration. If not set, all traces will be sent to the external service.
5. Ensure the OpenTelemetry Collector is running, start the Svix server, and watch the tracing information be received.
