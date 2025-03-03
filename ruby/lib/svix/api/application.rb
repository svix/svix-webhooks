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
          "limit" => options["limit"],
          "iterator" => options["iterator"],
          "order" => options["order"]
        }
      )
      ListResponseApplicationOut.deserialize(res)
    end

    def create(application_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
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
          "get_if_exists" => true
        },
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def get(app_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}"
      )
      ApplicationOut.deserialize(res)
    end

    def update(app_id, application_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}",
        body: application_in
      )
      ApplicationOut.deserialize(res)
    end

    def delete(app_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/app/#{app_id}"
      )
    end

    def patch(app_id, application_patch)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}",
        body: application_patch
      )
      ApplicationOut.deserialize(res)
    end

  end
end
