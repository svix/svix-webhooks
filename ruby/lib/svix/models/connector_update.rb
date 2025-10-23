# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ConnectorUpdate
    attr_accessor :allowed_event_types
    attr_accessor :description
    attr_accessor :feature_flags
    attr_accessor :instructions
    attr_accessor :kind
    attr_accessor :logo
    attr_accessor :name
    attr_accessor :transformation

    ALL_FIELD ||= [
      "allowed_event_types",
      "description",
      "feature_flags",
      "instructions",
      "kind",
      "logo",
      "name",
      "transformation"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ConnectorUpdate` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ConnectorUpdate")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["allowed_event_types"] = attributes["allowedEventTypes"]
      attrs["description"] = attributes["description"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["instructions"] = attributes["instructions"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"]) if attributes["kind"]
      attrs["logo"] = attributes["logo"]
      attrs["name"] = attributes["name"]
      attrs["transformation"] = attributes["transformation"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["allowedEventTypes"] = Svix::serialize_primitive(@allowed_event_types) if @allowed_event_types
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["instructions"] = Svix::serialize_primitive(@instructions) if @instructions
      out["kind"] = Svix::serialize_schema_ref(@kind) if @kind
      out["logo"] = Svix::serialize_primitive(@logo) if @logo
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["transformation"] = Svix::serialize_primitive(@transformation) if @transformation
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
