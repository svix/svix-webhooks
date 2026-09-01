# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class DestinationAutoconfig
    def initialize(client)
      @client = client
    end

    def subscribe(app_id, autoconfig_id, destination_in)
      res = @client.execute_request(
        "PUT",
        "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}/destination",
        body: destination_in
      )
      DestinationOut.deserialize(res)
    end

  end
end
