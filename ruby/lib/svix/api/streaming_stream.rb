# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamingStream
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseStreamOut.deserialize(res)
    end

    def create(stream_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: stream_in
      )
      StreamOut.deserialize(res)
    end

    def get(stream_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      StreamOut.deserialize(res)
    end

    def update(stream_id, stream_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/#{stream_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: stream_in
      )
      StreamOut.deserialize(res)
    end

    def delete(stream_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/stream/#{stream_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(stream_id, stream_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: stream_patch
      )
      StreamOut.deserialize(res)
    end

  end
end
