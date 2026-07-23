# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class OperationalWebhookEndpoint
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/operational-webhook/endpoint",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseOperationalWebhookEndpointOut.deserialize(res)
    end

    def create(operational_webhook_endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/operational-webhook/endpoint",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: operational_webhook_endpoint_in
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def get(endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def update(endpoint_id, operational_webhook_endpoint_update, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: operational_webhook_endpoint_update
      )
      OperationalWebhookEndpointOut.deserialize(res)
    end

    def delete(endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def get_secret(endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}/secret",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      OperationalWebhookEndpointSecretOut.deserialize(res)
    end

    def rotate_secret(endpoint_id, operational_webhook_endpoint_secret_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}/secret/rotate",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: operational_webhook_endpoint_secret_in
      )
    end

    def get_headers(endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      OperationalWebhookEndpointHeadersOut.deserialize(res)
    end

    def update_headers(endpoint_id, operational_webhook_endpoint_headers_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PUT",
        "/api/v1/operational-webhook/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: operational_webhook_endpoint_headers_in
      )
    end

  end
end
