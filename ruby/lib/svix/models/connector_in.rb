# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ConnectorIn
    attr_accessor :name
    # The Connector's UID.
    attr_accessor :uid
    attr_accessor :logo
    attr_accessor :description
    attr_accessor :kind
    attr_accessor :instructions
    attr_accessor :allowed_event_types
    attr_accessor :transformation
    attr_accessor :feature_flags
    attr_accessor :product_type

    ALL_FIELD ||= [
      "name",
      "uid",
      "logo",
      "description",
      "kind",
      "instructions",
      "allowed_event_types",
      "transformation",
      "feature_flags",
      "product_type"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ConnectorIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ConnectorIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["name"] = attributes["name"]
      attrs["uid"] = attributes["uid"]
      attrs["logo"] = attributes["logo"]
      attrs["description"] = attributes["description"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"]) if attributes["kind"]
      attrs["instructions"] = attributes["instructions"]
      attrs["allowed_event_types"] = attributes["allowedEventTypes"]
      attrs["transformation"] = attributes["transformation"]
      attrs["feature_flags"] = attributes["featureFlags"]
      if attributes["productType"]
        attrs["product_type"] = Svix::ConnectorProduct.deserialize(attributes["productType"])
      end

      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["logo"] = Svix::serialize_primitive(@logo) if @logo
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["kind"] = Svix::serialize_schema_ref(@kind) if @kind
      out["instructions"] = Svix::serialize_primitive(@instructions) if @instructions
      out["allowedEventTypes"] = Svix::serialize_primitive(@allowed_event_types) if @allowed_event_types
      out["transformation"] = Svix::serialize_primitive(@transformation) if @transformation
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["productType"] = Svix::serialize_schema_ref(@product_type) if @product_type
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
