# frozen_string_literal: true

require "svix/models/auto_config_sink_type"
require "svix/models/poller_v2_commit_in"
require "svix/models/sink_in_common"
require "svix/models/subscribe_in"
require "svix/api_internal/endpoint_auto_config"
require "svix/api_internal/message_pollerv2"

module Svix
  class AutoConfigConsumer
    def initialize(token, sink_in)
      content = AutoConfig.decode_token!(token)
      @app_id = content.fetch("app_id")
      @sink_id = content.fetch("endpoint_id")
      @sink_in = sink_in
      @client = SvixHttpClient.new(
        content.fetch("token_plaintext"),
        URI(content.fetch("server_url"))
      )
    end

    def subscribe
      poller = AutoConfigSinkTypeConfig::Poller.new(@sink_in.serialize)
      subscribe_in = SubscribeIn.new(
        "sink" => AutoConfigSinkType.new("config" => poller)
      )
      EndpointAutoConfig.new(@client).update(@app_id, @sink_id, subscribe_in)
    end

    def receive(consumer_id, options = {})
      MessagePollerv2.new(@client).consumer_poll(
        @app_id,
        @sink_id,
        consumer_id,
        options
      )
    end

    def commit(consumer_id, offset, options = {})
      MessagePollerv2.new(@client).consumer_commit(
        @app_id,
        @sink_id,
        consumer_id,
        PollerV2CommitIn.new("offset" => offset),
        options
      )
    end
  end
end
