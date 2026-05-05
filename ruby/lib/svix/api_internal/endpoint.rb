# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Endpoint
    attr_accessor :auto_config
    def initialize(client)
      @client = client
      @auto_config = EndpointAutoConfig.new(client)
    end

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        body: endpoint_transformation_in
      )
    end

  end
end
