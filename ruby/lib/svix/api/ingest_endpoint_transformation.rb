# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class IngestEndpointTransformation
    def initialize(client)
      @client = client
    end

    def transformation(source_id, endpoint_id)
      res = @client.execute_request(
        "GET",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/transformation"
      )
      IngestEndpointTransformationOut.deserialize(res)
    end

    def patch(source_id, endpoint_id, ingest_endpoint_transformation_patch)
      @client.execute_request(
        "PATCH",
        "/ingest/api/v1/source/#{source_id}/endpoint/#{endpoint_id}/transformation",
        body: ingest_endpoint_transformation_patch
      )
    end

  end
end
