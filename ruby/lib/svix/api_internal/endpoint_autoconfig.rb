# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointAutoconfig
    def initialize(client)
      @client = client
    end

    def subscribe(app_id, autoconfig_id, endpoint_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}/endpoint",
        body: endpoint_in
      )
      EndpointOut.deserialize(res)
    end

  end
end
