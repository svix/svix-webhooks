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

    def transformation_partial_update(app_id, endpoint_id, endpoint_transformation_in, options = {})
      options = options.transform_keys(&:to_s)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        headers: {
          "x-request-id" => options["request_id"]
        }.compact,
        body: endpoint_transformation_in
      )
    end

  end
end
