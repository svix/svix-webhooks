# frozen_string_literal: true
# This file is @generated

require "net/http"

module Svix
  class Stream
    attr_accessor :event_type
    attr_accessor :events
    attr_accessor :sink
    attr_accessor :stream
    def initialize(client)
      @client = client
      @event_type = StreamEventType.new(client)
      @events = StreamEvents.new(client)
      @sink = StreamSink.new(client)
      @stream = StreamStream.new(client)
    end

    def sink_headers_get(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/headers"
      )
      EndpointHeadersOut.deserialize(res)
    end

    def sink_headers_patch(stream_id, sink_id, http_sink_headers_patch_in)
      res = @client.execute_request(
        "PATCH",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/headers",
        body: http_sink_headers_patch_in
      )
      EndpointHeadersOut.deserialize(res)
    end

    def sink_transformation_get(stream_id, sink_id)
      res = @client.execute_request(
        "GET",
        "/api/v1/stream/#{stream_id}/sink/#{sink_id}/transformation"
      )
      SinkTransformationOut.deserialize(res)
    end

  end
end
