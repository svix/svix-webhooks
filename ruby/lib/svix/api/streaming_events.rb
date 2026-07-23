# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamingEvents
    def initialize(client)
      @client = client
    end

    def get(stream_id, sink_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/events",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "after" => options["after"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EventStreamOut.deserialize(res)
    end

    def create(stream_id, create_stream_events_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/events",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: create_stream_events_in
      )
      CreateStreamEventsOut.deserialize(res)
    end

  end
end
