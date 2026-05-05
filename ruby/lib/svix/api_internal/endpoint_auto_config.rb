# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointAutoConfig
    def initialize(client)
      @client = client
    end

    def update(app_id, endpoint_id, subscribe_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/auto-config",
        body: subscribe_in
      )
      EndpointOut.deserialize(res)
    end

  end
end
