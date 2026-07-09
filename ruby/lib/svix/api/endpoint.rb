# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Endpoint
    def initialize(client)
      @client = client
    end

    def list(app_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint",
        query_params: {
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseEndpointOut.deserialize(res)
    end

    def create(app_id, endpoint_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: endpoint_in
      )
      EndpointOut.deserialize(res)
    end

    def get(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EndpointOut.deserialize(res)
    end

    def update(app_id, endpoint_id, endpoint_update, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_update
      )
      EndpointOut.deserialize(res)
    end

    def delete(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(app_id, endpoint_id, endpoint_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_patch
      )
      EndpointOut.deserialize(res)
    end

    def get_secret(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/secret",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EndpointSecretOut.deserialize(res)
    end

    def rotate_secret(app_id, endpoint_id, endpoint_secret_rotate_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/secret/rotate",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: endpoint_secret_rotate_in
      )
    end

    def get_headers(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EndpointHeadersOut.deserialize(res)
    end

    def update_headers(app_id, endpoint_id, endpoint_headers_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_headers_in
      )
    end

    def patch_headers(app_id, endpoint_id, endpoint_headers_patch_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/headers",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_headers_patch_in
      )
    end

    def transformation_get(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EndpointTransformationOut.deserialize(res)
    end

    def patch_transformation(app_id, endpoint_id, endpoint_transformation_patch, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_transformation_patch
      )
    end

    def replay_missing(app_id, endpoint_id, replay_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/replay-missing",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: replay_in
      )
      ReplayOut.deserialize(res)
    end

    def bulk_replay(app_id, endpoint_id, bulk_replay_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/bulk-replay",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: bulk_replay_in
      )
      ReplayOut.deserialize(res)
    end

    def get_stats(app_id, endpoint_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/stats",
        query_params: {
          "since" => options["since"],
          "until" => options["until"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      EndpointStats.deserialize(res)
    end

    def recover(app_id, endpoint_id, recover_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/recover",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: recover_in
      )
      RecoverOut.deserialize(res)
    end

    def send_example(app_id, endpoint_id, event_example_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/send-example",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: event_example_in
      )
      MessageOut.deserialize(res)
    end

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_transformation_in
      )
    end

  end
end
