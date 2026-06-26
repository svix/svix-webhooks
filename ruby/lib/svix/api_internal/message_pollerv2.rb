# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class MessagePollerv2
    def initialize(client)
      @client = client
    end

    def consumer_poll(app_id, sink_id, consumer_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/polling-endpoint/#{sink_id}/consumer/#{consumer_id}",
        query_params: {
          "limit" => options["limit"],
          "lease_duration_ms" => options["lease_duration_ms"],
          "starting_position" => options["starting_position"]
        }
      )
      PollerV2PollOut.deserialize(res)
    end

    def consumer_commit(app_id, sink_id, consumer_id, poller_v2_commit_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/polling-endpoint/#{sink_id}/consumer/#{consumer_id}/commit",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: poller_v2_commit_in
      )
    end

  end
end
