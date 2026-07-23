# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EnvironmentIn
    attr_accessor :event_types
    attr_accessor :settings
    attr_accessor :connectors

    ALL_FIELD ||= ["event_types", "settings", "connectors"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EnvironmentIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EnvironmentIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      if attributes["eventTypes"]
        attrs["event_types"] = attributes["eventTypes"].map { |v| Svix::EventTypeIn.deserialize(v) }
      end

      attrs["settings"] = attributes["settings"]
      if attributes["connectors"]
        attrs["connectors"] = attributes["connectors"].map { |v| Svix::ConnectorIn.deserialize(v) }
      end

      new(attrs)
    end

    def serialize
      out = Hash.new
      out["eventTypes"] = @event_types.map { |v| v.serialize } if @event_types
      out["settings"] = Svix::serialize_primitive(@settings) if @settings
      out["connectors"] = @connectors.map { |v| v.serialize } if @connectors
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
