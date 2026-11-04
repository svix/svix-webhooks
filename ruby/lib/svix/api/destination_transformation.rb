# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class DestinationTransformation
    def initialize(client)
      @client = client
    end

    def get(app_id, destination_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/app/#{app_id}/destination/#{destination_id}/transformation"
      )
      DestinationTransformationOut.deserialize(res)
    end

    def patch(app_id, destination_id, destination_transform_in)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/app/#{app_id}/destination/#{destination_id}/transformation",
        body: destination_transform_in
      )
      EmptyResponse.deserialize(res)
    end

  end
end
