# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessagePrecheckIn
    attr_accessor :channels
    # The event type's name
    attr_accessor :event_type

    ALL_FIELD ||= ["channels", "event_type"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessagePrecheckIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessagePrecheckIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["channels"] = attributes["channels"]
      attrs["event_type"] = attributes["eventType"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
