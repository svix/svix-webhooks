# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamingSink
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
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseStreamSinkOut.deserialize(res)
    end

    def create(stream_id, stream_sink_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/sink",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: stream_sink_in
      )
      StreamSinkOut.deserialize(res)
    end

    def get(stream_id, sink_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      StreamSinkOut.deserialize(res)
    end

    def update(stream_id, sink_id, stream_sink_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: stream_sink_in
      )
      StreamSinkOut.deserialize(res)
    end

    def delete(stream_id, sink_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(stream_id, sink_id, stream_sink_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: stream_sink_patch
      )
      StreamSinkOut.deserialize(res)
    end

    def transformation_partial_update(stream_id, sink_id, sink_transform_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: sink_transform_in
      )
      EmptyResponse.deserialize(res)
    end

    def get_secret(stream_id, sink_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/secret",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      SinkSecretOut.deserialize(res)
    end

    def rotate_secret(stream_id, sink_id, endpoint_secret_rotate_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/secret/rotate",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: endpoint_secret_rotate_in
      )
      EmptyResponse.deserialize(res)
    end

  end
end
