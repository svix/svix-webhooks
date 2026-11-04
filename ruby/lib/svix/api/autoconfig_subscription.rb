# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class AutoconfigSubscription
    def initialize(client)
      @client = client
    end

    def create(app_id, create_auto_config_subscription_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "POST",
        "/api/v1/app/#{app_id}/autoconfig",
        headers: {
          "idempotency-key" => options["idempotency-key"]
        },
        body: create_auto_config_subscription_in
      )
      AutoConfigOut.deserialize(res)
    end

    def delete(app_id, autoconfig_id)
      @client.execute_request(
        "DELETE",
        "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}"
      )
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
