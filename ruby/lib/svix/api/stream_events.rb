# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamEvents
    def initialize(client)
      @client = client
    end

    def create(stream_id, create_stream_events_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/events",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: create_stream_events_in
      )
      CreateStreamEventsOut.deserialize(res)
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
        }
      )
      EventStreamOut.deserialize(res)
    end

  end
end
