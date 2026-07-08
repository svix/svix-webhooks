# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class EndpointTransformation
    def initialize(client)
      @client = client
    end

    def get(app_id, endpoint_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation"
      )
      EndpointTransformationOut.deserialize(res)
    end

    def patch(app_id, endpoint_id, endpoint_transformation_patch)
      @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/endpoint/#{endpoint_id}/transformation",
        body: endpoint_transformation_patch
      )
    end

  end
end
