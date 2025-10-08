// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

use super::{
    stream_event_type::StreamEventTypeArgs, stream_events::StreamEventsArgs,
    stream_sink::StreamSinkArgs, stream_stream::StreamStreamArgs,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamArgs {
    #[command(subcommand)]
    pub command: StreamCommands,
}

#[derive(Subcommand)]
pub enum StreamCommands {
    EventType(StreamEventTypeArgs),
    Events(StreamEventsArgs),
    Sink(StreamSinkArgs),
    Stream(StreamStreamArgs),
    /// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
    SinkHeadersGet {
        id: String,
        sink_id: String,
    },
    /// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
    SinkHeadersPatch {
        id: String,
        sink_id: String,
        http_sink_headers_patch_in: crate::json::JsonOf<HttpSinkHeadersPatchIn>,
    },
    /// Get the transformation code associated with this sink.
    SinkTransformationGet {
        id: String,
        sink_id: String,
    },
}

impl StreamCommands {
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
            Self::SinkHeadersGet { id, sink_id } => {
                let resp = client.stream().sink_headers_get(id, sink_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::SinkHeadersPatch {
                id,
                sink_id,
                http_sink_headers_patch_in,
            } => {
                let resp = client
                    .stream()
                    .sink_headers_patch(id, sink_id, http_sink_headers_patch_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::SinkTransformationGet { id, sink_id } => {
                let resp = client.stream().sink_transformation_get(id, sink_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
