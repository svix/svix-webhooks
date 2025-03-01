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
      path = "/ingest/api/v1/source/{source_id}/endpoint"
      res = @client.execute_request(
        "GET",
        path,
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
      path = "/ingest/api/v1/source/{source_id}/endpoint"
      res = @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_endpoint_in
      )
      IngestEndpointOut.deserialize(res)
    end

    def get(endpoint_id)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "GET",
        path
      )
      IngestEndpointOut.deserialize(res)
    end

    def update(endpoint_id, ingest_endpoint_update)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}"
      res = @client.execute_request(
        "PUT",
        path,
        body: ingest_endpoint_update
      )
      IngestEndpointOut.deserialize(res)
    end

    def delete(endpoint_id)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}"
      @client.execute_request(
        "DELETE",
        path
      )
    end

    def get_headers(endpoint_id)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/headers"
      res = @client.execute_request(
        "GET",
        path
      )
      IngestEndpointHeadersOut.deserialize(res)
    end

    def update_headers(endpoint_id, ingest_endpoint_headers_in)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/headers"
      @client.execute_request(
        "PUT",
        path,
        body: ingest_endpoint_headers_in
      )
    end

    def get_secret(endpoint_id)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/secret"
      res = @client.execute_request(
        "GET",
        path
      )
      IngestEndpointSecretOut.deserialize(res)
    end

    def rotate_secret(endpoint_id, ingest_endpoint_secret_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/ingest/api/v1/source/{source_id}/endpoint/#{endpoint_id}/secret/rotate"
      @client.execute_request(
        "POST",
        path,
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: ingest_endpoint_secret_in
      )
    end

  end
end
