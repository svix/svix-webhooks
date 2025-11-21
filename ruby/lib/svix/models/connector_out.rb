# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ConnectorOut
    attr_accessor :allowed_event_types
    attr_accessor :created_at
    attr_accessor :description
    attr_accessor :feature_flags
    # The Connector's ID.
    attr_accessor :id
    attr_accessor :instructions
    attr_accessor :kind
    attr_accessor :logo
    attr_accessor :name
    # The Environment's ID.
    attr_accessor :org_id
    attr_accessor :product_type
    attr_accessor :transformation
    # The Connector's UID.
    attr_accessor :uid
    attr_accessor :updated_at

    ALL_FIELD ||= [
      "allowed_event_types",
      "created_at",
      "description",
      "feature_flags",
      "id",
      "instructions",
      "kind",
      "logo",
      "name",
      "org_id",
      "product_type",
      "transformation",
      "uid",
      "updated_at"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ConnectorOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ConnectorOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["allowed_event_types"] = attributes["allowedEventTypes"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["description"] = attributes["description"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["id"] = attributes["id"]
      attrs["instructions"] = attributes["instructions"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"])
      attrs["logo"] = attributes["logo"]
      attrs["name"] = attributes["name"]
      attrs["org_id"] = attributes["orgId"]
      attrs["product_type"] = Svix::ConnectorProduct.deserialize(attributes["productType"])
      attrs["transformation"] = attributes["transformation"]
      attrs["uid"] = attributes["uid"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["allowedEventTypes"] = Svix::serialize_primitive(@allowed_event_types) if @allowed_event_types
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["instructions"] = Svix::serialize_primitive(@instructions) if @instructions
      out["kind"] = Svix::serialize_schema_ref(@kind) if @kind
      out["logo"] = Svix::serialize_primitive(@logo) if @logo
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["orgId"] = Svix::serialize_primitive(@org_id) if @org_id
      out["productType"] = Svix::serialize_schema_ref(@product_type) if @product_type
      out["transformation"] = Svix::serialize_primitive(@transformation) if @transformation
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
