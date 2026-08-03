# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EnvironmentOut
    attr_accessor :version
    attr_accessor :created_at
    attr_accessor :event_types
    attr_accessor :settings
    attr_accessor :connectors

    ALL_FIELD ||= ["version", "created_at", "event_types", "settings", "connectors"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EnvironmentOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EnvironmentOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["version"] = attributes["version"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["event_types"] = attributes["eventTypes"].map { |v| Svix::EventTypeOut.deserialize(v) }
      attrs["settings"] = attributes["settings"]
      attrs["connectors"] = attributes["connectors"].map { |v| Svix::ConnectorOut.deserialize(v) }
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["version"] = Svix::serialize_primitive(@version) unless @version.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["eventTypes"] = @event_types.map { |v| v.serialize } unless @event_types.nil?
      out["settings"] = Svix::serialize_primitive(@settings) unless @settings.nil?
      out["connectors"] = @connectors.map { |v| v.serialize } unless @connectors.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
