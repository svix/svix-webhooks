# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  module ApiInternal
    class Autoconfig
      attr_accessor :destination
      attr_accessor :endpoint
      def initialize(client)
        @client = client
        @destination = AutoconfigDestination.new(client)
        @endpoint = AutoconfigEndpoint.new(client)
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
end
