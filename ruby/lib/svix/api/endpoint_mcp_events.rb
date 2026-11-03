# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointMcpEvents
    def initialize(client)
      @client = client
    end

    def subscribe(app_id, mcp_events_subscribe_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/mcp-events/subscribe",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: mcp_events_subscribe_in
      )
      McpEventsSubscribeOut.deserialize(res)
    end

    def unsubscribe(app_id, mcp_events_unsubscribe_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/mcp-events/unsubscribe",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: mcp_events_unsubscribe_in
      )
    end

  end
end
