# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class AutoconfigSubscription
    attr_accessor :destination
    attr_accessor :endpoint
    def initialize(client)
      @client = client
      @destination = AutoconfigSubscriptionDestination.new(client)
      @endpoint = AutoconfigSubscriptionEndpoint.new(client)
    end

    def get(app_id, autoconfig_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}"
      )
      AutoConfigSubscriptionOut.deserialize(res)
    end

  end
end
