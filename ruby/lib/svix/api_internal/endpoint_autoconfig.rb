# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  module ApiInternal
    class EndpointAutoconfig
      def initialize(client)
        @client = client
      end

      def get(app_id, autoconfig_id)
        res = @client.execute_request(
          "GET",
          "/api/v1/app/#{app_id}/autoconfig/#{autoconfig_id}"
        )
        AutoConfigSubscriptionOut.deserialize(res)
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
end
