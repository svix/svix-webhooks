# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Environment
    def initialize(client)
      @client = client
    end

    def export(options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/environment/export",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact
      )
      EnvironmentOut.deserialize(res)
    end

    def import(environment_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "POST",
        "/api/v1/environment/import",
        headers: {
          "x-request-id" => options["request_id"],
          "idempotency-key" => options["idempotency-key"]
        }.compact,
        body: environment_in
      )
    end

  end
end
