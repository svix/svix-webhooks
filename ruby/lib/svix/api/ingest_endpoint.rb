# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class IngestEndpoint
    def initialize(client)
      @client = client
    end

    def list(source_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseIngestEndpointOut.deserialize(res)
    end

    def create(source_id, ingest_endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/ingest/api/v1/source/#{source_id}/endpoint",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: ingest_endpoint_in
      )
      IngestEndpointOut.deserialize(res)
    end

    def get(source_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      IngestEndpointOut.deserialize(res)
    end

    def update(source_id, endpoint_id, ingest_endpoint_update, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: ingest_endpoint_update
      )
      IngestEndpointOut.deserialize(res)
    end

    def delete(source_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def get_secret(source_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/secret",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      IngestEndpointSecretOut.deserialize(res)
    end

    def rotate_secret(source_id, endpoint_id, ingest_endpoint_secret_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/secret/rotate",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: ingest_endpoint_secret_in
      )
    end

    def get_headers(source_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      IngestEndpointHeadersOut.deserialize(res)
    end

    def update_headers(source_id, endpoint_id, ingest_endpoint_headers_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PUT",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: ingest_endpoint_headers_in
      )
    end

    def get_transformation(source_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      IngestEndpointTransformationOut.deserialize(res)
    end

    def set_transformation(source_id, endpoint_id, ingest_endpoint_transformation_patch, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PATCH",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: ingest_endpoint_transformation_patch
      )
    end

  end
end
