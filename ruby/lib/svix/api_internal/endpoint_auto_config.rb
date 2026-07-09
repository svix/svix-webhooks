# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointAutoConfig
    def initialize(client)
      @client = client
    end

    def update(app_id, endpoint_id, subscribe_in, options = {})
      options = options.transform_keys(&:to_s)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/auto-config",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: subscribe_in
      )
      EndpointOut.deserialize(res)
    end

  end
end
