# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class IngestEndpoint
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/{source_id}/endpoint",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseIngestEndpointOut.deserialize(res)
    end

    def create(ingest_endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source/{source_id}/endpoint",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_endpoint_in
      )
      IngestEndpointOut.deserialize(res)
    end

    def get(endpoint_id)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}"
      )
      IngestEndpointOut.deserialize(res)
    end

    def update(endpoint_id, ingest_endpoint_update)
      res = @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}",
        body: ingest_endpoint_update
      )
      IngestEndpointOut.deserialize(res)
    end

    def delete(endpoint_id)
      @client.execute_request(
        "DELETE",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}"
      )
    end

    def get_headers(endpoint_id)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/headers"
      )
      IngestEndpointHeadersOut.deserialize(res)
    end

    def update_headers(endpoint_id, ingest_endpoint_headers_in)
      @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/headers",
        body: ingest_endpoint_headers_in
      )
    end

    def get_secret(endpoint_id)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/secret"
      )
      IngestEndpointSecretOut.deserialize(res)
    end

    def rotate_secret(endpoint_id, ingest_endpoint_secret_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/secret/rotate",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_endpoint_secret_in
      )
    end

  end
end
