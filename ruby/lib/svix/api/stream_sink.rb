# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamSink
    def initialize(client)
      @client = client
    end

    def list(stream_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseStreamSinkOut.deserialize(res)
    end

    def create(stream_id, stream_sink_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/sink",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: stream_sink_in
      )
      StreamSinkOut.deserialize(res)
    end

    def get(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}"
      )
      StreamSinkOut.deserialize(res)
    end

    def update(stream_id, sink_id, stream_sink_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        body: stream_sink_in
      )
      StreamSinkOut.deserialize(res)
    end

    def delete(stream_id, sink_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}"
      )
    end

    def patch(stream_id, sink_id, stream_sink_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        body: stream_sink_patch
      )
      StreamSinkOut.deserialize(res)
    end

    def get_secret(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/secret"
      )
      SinkSecretOut.deserialize(res)
    end

    def rotate_secret(stream_id, sink_id, endpoint_secret_rotate_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/secret/rotate",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: endpoint_secret_rotate_in
      )
      EmptyResponse.deserialize(res)
    end

    def transformation_partial_update(stream_id, sink_id, sink_transform_in)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/transformation",
        body: sink_transform_in
      )
      EmptyResponse.deserialize(res)
    end

  end
end
