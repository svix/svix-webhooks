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
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseStreamEventTypeOut.deserialize(res)
    end

    def create(stream_event_type_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/event-type",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: stream_event_type_in
      )
      StreamEventTypeOut.deserialize(res)
    end

    def get(name, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/event-type/#{name}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      StreamEventTypeOut.deserialize(res)
    end

    def update(name, stream_event_type_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/event-type/#{name}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
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
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(name, stream_event_type_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/event-type/#{name}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: stream_event_type_patch
      )
      StreamEventTypeOut.deserialize(res)
    end

  end
end
