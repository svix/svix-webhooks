# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class CreateStreamEventsIn
    attr_accessor :events
    # Optionally creates a new Stream alongside the events.
    #
    # If the stream id or uid that is used in the path already exists, this argument is ignored.
    attr_accessor :stream

    ALL_FIELD ||= ["events", "stream"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::CreateStreamEventsIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::CreateStreamEventsIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["events"] = attributes["events"].map { |v| Svix::EventIn.deserialize(v) }
      attrs["stream"] = Svix::StreamIn.deserialize(attributes["stream"]) if attributes["stream"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["events"] = @events.map { |v| v.serialize } if @events
      out["stream"] = Svix::serialize_schema_ref(@stream) if @stream
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
