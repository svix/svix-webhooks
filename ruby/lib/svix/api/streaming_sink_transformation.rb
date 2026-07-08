# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class StreamingSinkTransformation
    def initialize(client)
      @client = client
    end

    def get(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/transformation"
      )
      SinkTransformationOut.deserialize(res)
    end

    def patch(stream_id, sink_id, sink_transform_in)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/transformation",
        body: sink_transform_in
      )
      EmptyResponse.deserialize(res)
    end

  end
end
