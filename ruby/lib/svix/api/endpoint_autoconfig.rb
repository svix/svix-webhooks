# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointAutoconfig
    def initialize(client)
      @client = client
    end

    def create(app_id, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/autoconfig",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        }
      )
      AutoConfigOut.deserialize(res)
    end

    def rotate(app_id, autoconfig_id, rotate_subscription_in2, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}/rotate",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: rotate_subscription_in2
      )
      AutoConfigOut.deserialize(res)
    end

  end
end
