# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ConnectorPatch
    attr_accessor :name
    attr_accessor :logo
    attr_accessor :description
    attr_accessor :kind
    attr_accessor :instructions
    attr_accessor :allowed_event_types
    attr_accessor :transformation
    attr_accessor :feature_flags

    ALL_FIELD ||= [
      "name",
      "logo",
      "description",
      "kind",
      "instructions",
      "allowed_event_types",
      "transformation",
      "feature_flags"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ConnectorPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ConnectorPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["name"] = attributes["name"]
      attrs["logo"] = attributes["logo"]
      attrs["description"] = attributes["description"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"]) if attributes["kind"]
      attrs["instructions"] = attributes["instructions"]
      attrs["allowed_event_types"] = attributes["allowedEventTypes"]
      attrs["transformation"] = attributes["transformation"]
      attrs["feature_flags"] = attributes["featureFlags"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["logo"] = Svix::serialize_primitive(@logo) if @__logo_is_defined
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["kind"] = Svix::serialize_schema_ref(@kind) if @kind
      out["instructions"] = Svix::serialize_primitive(@instructions) if @instructions
      out["allowedEventTypes"] = Svix::serialize_primitive(@allowed_event_types) if @__allowed_event_types_is_defined
      out["transformation"] = Svix::serialize_primitive(@transformation) if @transformation
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @__feature_flags_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
