# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventOut
    # The event type's name
    attr_accessor :event_type
    attr_accessor :payload
    attr_accessor :timestamp

    ALL_FIELD ||= ["event_type", "payload", "timestamp"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["event_type"] = attributes["eventType"]
      attrs["payload"] = attributes["payload"]
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["timestamp"] = Svix::serialize_primitive(@timestamp) if @timestamp
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
