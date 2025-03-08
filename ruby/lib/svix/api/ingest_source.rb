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
        }
      )
      ListResponseIngestSourceOut.deserialize(res)
    end

    def create(ingest_source_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_source_in
      )
      IngestSourceOut.deserialize(res)
    end

    def get(source_id)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}"
      )
      IngestSourceOut.deserialize(res)
    end

    def update(source_id, ingest_source_in)
      res = @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/#{source_id}",
        body: ingest_source_in
      )
      IngestSourceOut.deserialize(res)
    end

    def delete(source_id)
      @client.execute_request(
        "DELETE",
        "/ingest/api/v1/source/#{source_id}"
      )
    end

    def rotate_token(source_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source/#{source_id}/token/rotate",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      RotateTokenOut.deserialize(res)
    end

  end
end
