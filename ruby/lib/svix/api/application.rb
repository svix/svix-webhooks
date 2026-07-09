# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Application
    def initialize(client)
      @client = client
    end

    def list(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app",
        query_params: {
          "exclude_apps_with_no_endpoints" => options["exclude_apps_with_no_endpoints"],
          "exclude_apps_with_disabled_endpoints" => options["exclude_apps_with_disabled_endpoints"],
          "exclude_apps_with_svix_play_endpoints" => options["exclude_apps_with_svix_play_endpoints"],
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        },
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ListResponseApplicationOut.deserialize(res)
    end

    def create(application_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def get_or_create(application_in, options = {})
      options = options.transform_keys(&:to_s)
      path = "/api/v1/app"
      res = @client.execute_request(
        "POST",
        path,
        query_params: {
          "get_if_exists" => "true"
        },
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def get(app_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
      ApplicationOut.deserialize(res)
    end

    def update(app_id, application_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def delete(app_id, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "DELETE",
        "/api/v1/app/#{app_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact
      )
    end

    def patch(app_id, application_patch, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: application_patch
      )
      ApplicationOut.deserialize(res)
    end

  end
end
