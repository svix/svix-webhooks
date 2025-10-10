# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamingEventType
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/event-type",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"],
          "include_archived" => options["include_archived"]
        }
      )
      ListResponseStreamEventTypeOut.deserialize(res)
    end

    def create(stream_event_type_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/event-type",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: stream_event_type_in
      )
      StreamEventTypeOut.deserialize(res)
    end

    def get(name)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/event-type/#{name}"
      )
      StreamEventTypeOut.deserialize(res)
    end

    def update(name, stream_event_type_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/event-type/#{name}",
        body: stream_event_type_in
      )
      StreamEventTypeOut.deserialize(res)
    end

    def delete(name, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/stream/event-type/#{name}",
        query_params: {
          "expunge" => options["expunge"]
        }
      )
    end

    def patch(name, stream_event_type_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/event-type/#{name}",
        body: stream_event_type_patch
      )
      StreamEventTypeOut.deserialize(res)
    end

  end
end
