# frozen_string_literal: true

require "uri"

require "svix/models/auto_config_sink_type"
require "svix/models/destination_in"
require "svix/models/destination_out"
require "svix/models/poller_v2_commit_in"
require "svix/models/sink_in_common"
require "svix/models/sink_status"
require "svix/models/subscribe_in"
require "svix/api_internal/endpoint_auto_config_deprecated"
require "svix/api_internal/destination_autoconfig"
require "svix/api_internal/message_pollerv2"

module Svix
  class AutoConfigConsumer
    def initialize(token, sink_in)
      content = AutoConfig.decode_token!(token)
      @app_id = content.fetch("app_id")
      @sink_id = content["endpoint_id"]
      @autoconfig_id = content["autoconfig_id"]
      @sink_in = sink_in
      @client = SvixHttpClient.new(
        content.fetch("token_plaintext"),
        URI(content.fetch("server_url"))
      )
    end

    def subscribe
      if @autoconfig_id
        destination = DestinationAutoconfig.new(@client).subscribe(
          @app_id,
          @autoconfig_id,
          sink_in_common_to_polling_destination(@sink_in)
        )
        @sink_id = destination.id
        destination
      else
        poller = AutoConfigSinkTypeConfig::Poller.deserialize(@sink_in.serialize)
        subscribe_in = SubscribeIn.new(
          "sink" => AutoConfigSinkType.new("config" => poller)
        )
        endpoint = EndpointAutoConfigDeprecated.new(@client).update(@app_id, @sink_id, subscribe_in)
        destination_out_from_v1_endpoint(endpoint)
      end
    end

    def receive(consumer_id, options = {})
      @sink_id ||= subscribe.id
      MessagePollerv2.new(@client).consumer_poll(
        @app_id,
        @sink_id,
        consumer_id,
        options
      )
    end

    def commit(consumer_id, offset, options = {})
      @sink_id ||= subscribe.id
      MessagePollerv2.new(@client).consumer_commit(
        @app_id,
        @sink_id,
        consumer_id,
        PollerV2CommitIn.new("offset" => offset),
        options
      )
    end

    private

    def destination_out_from_v1_endpoint(endpoint)
      DestinationOut.new(
        "id" => endpoint.id,
        "uid" => endpoint.uid,
        "status" => endpoint.disabled ? SinkStatus::DISABLED : SinkStatus::ENABLED,
        "current_iterator" => "",
        "created_at" => endpoint.created_at,
        "updated_at" => endpoint.updated_at,
        "batch_size" => 0,
        "max_wait_secs" => 0,
        "event_types" => endpoint.event_types,
        "channels" => endpoint.channels,
        "metadata" => endpoint.metadata,
        "config" => DestinationOutConfig::PollingEndpoint.new
      )
    end

    def sink_in_common_to_polling_destination(sink)
      attrs = {
        "config" => DestinationInConfig::PollingEndpoint.new
      }
      attrs["uid"] = sink.uid unless sink.uid.nil?
      attrs["event_types"] = sink.event_types unless sink.event_types.nil?
      attrs["channels"] = sink.channels unless sink.channels.nil?
      attrs["metadata"] = sink.metadata unless sink.metadata.nil?
      DestinationIn.new(attrs)
    end
  end
end
