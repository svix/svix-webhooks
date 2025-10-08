# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamStream
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
        }
      )
      ListResponseStreamOut.deserialize(res)
    end

    def create(stream_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: stream_in
      )
      StreamOut.deserialize(res)
    end

    def get(stream_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}"
      )
      StreamOut.deserialize(res)
    end

    def update(stream_id, stream_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/#{stream_id}",
        body: stream_in
      )
      StreamOut.deserialize(res)
    end

    def delete(stream_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/stream/#{stream_id}"
      )
    end

    def patch(stream_id, stream_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}",
        body: stream_patch
      )
      StreamOut.deserialize(res)
    end

  end
end
