// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

use super::{
    streaming_event_type::StreamingEventTypeArgs, streaming_events::StreamingEventsArgs,
    streaming_sink::StreamingSinkArgs, streaming_stream::StreamingStreamArgs,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingArgs {
    #[command(subcommand)]
    pub command: StreamingCommands,
}

#[derive(Subcommand)]
pub enum StreamingCommands {
    EventType(StreamingEventTypeArgs),
    Events(StreamingEventsArgs),
    Sink(StreamingSinkArgs),
    Stream(StreamingStreamArgs),
    /// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
    SinkHeadersGet {
        stream_id: String,
        sink_id: String,
    },
    /// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
    SinkHeadersPatch {
        stream_id: String,
        sink_id: String,
        http_sink_headers_patch_in: crate::json::JsonOf<HttpSinkHeadersPatchIn>,
    },
    /// Get the transformation code associated with this sink.
    SinkTransformationGet {
        stream_id: String,
        sink_id: String,
    },
}

impl StreamingCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::EventType(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Events(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Sink(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Stream(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::SinkHeadersGet { stream_id, sink_id } => {
                let resp = client
                    .streaming()
                    .sink_headers_get(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::SinkHeadersPatch {
                stream_id,
                sink_id,
                http_sink_headers_patch_in,
            } => {
                let resp = client
                    .streaming()
                    .sink_headers_patch(stream_id, sink_id, http_sink_headers_patch_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::SinkTransformationGet { stream_id, sink_id } => {
                let resp = client
                    .streaming()
                    .sink_transformation_get(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
