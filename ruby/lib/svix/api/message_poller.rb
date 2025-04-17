# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class MessagePoller
    def initialize(client)
      @client = client
    end

    def poll(app_id, sink_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/poller/#{sink_id}",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "event_type" => options["event_type"],
          "channel" => options["channel"],
          "after" => options["after"]
        }
      )
      PollingEndpointOut.deserialize(res)
    end

    def consumer_poll(app_id, sink_id, consumer_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/poller/#{sink_id}/consumer/#{consumer_id}",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"]
        }
      )
      PollingEndpointOut.deserialize(res)
    end

    def consumer_seek(app_id, sink_id, consumer_id, polling_endpoint_consumer_seek_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/poller/#{sink_id}/consumer/#{consumer_id}/seek",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: polling_endpoint_consumer_seek_in
      )
      PollingEndpointConsumerSeekOut.deserialize(res)
    end

  end
end
