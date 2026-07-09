# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class IngestSource
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseIngestSourceOut.deserialize(res)
    end

    def create(ingest_source_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: ingest_source_in
      )
      IngestSourceOut.deserialize(res)
    end

    def get(source_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      IngestSourceOut.deserialize(res)
    end

    def update(source_id, ingest_source_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/#{source_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: ingest_source_in
      )
      IngestSourceOut.deserialize(res)
    end

    def delete(source_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/ingest/api/v1/source/#{source_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def rotate_token(source_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source/#{source_id}/token/rotate",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact
      )
      RotateTokenOut.deserialize(res)
    end

  end
end
