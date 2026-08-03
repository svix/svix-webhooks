# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ConnectorOut
    # The Connector's ID.
    attr_accessor :id
    # The Environment's ID.
    attr_accessor :org_id
    # The Connector's UID.
    attr_accessor :uid
    attr_accessor :kind
    attr_accessor :name
    attr_accessor :logo
    attr_accessor :description
    attr_accessor :instructions
    attr_accessor :allowed_event_types
    attr_accessor :transformation
    attr_accessor :created_at
    attr_accessor :updated_at
    attr_accessor :transformation_updated_at
    attr_accessor :feature_flags
    attr_accessor :product_type

    ALL_FIELD ||= [
      "id",
      "org_id",
      "uid",
      "kind",
      "name",
      "logo",
      "description",
      "instructions",
      "allowed_event_types",
      "transformation",
      "created_at",
      "updated_at",
      "transformation_updated_at",
      "feature_flags",
      "product_type"
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
      attrs["id"] = attributes["id"]
      attrs["org_id"] = attributes["orgId"]
      attrs["uid"] = attributes["uid"]
      attrs["kind"] = Svix::ConnectorKind.deserialize(attributes["kind"])
      attrs["name"] = attributes["name"]
      attrs["logo"] = attributes["logo"]
      attrs["description"] = attributes["description"]
      attrs["instructions"] = attributes["instructions"]
      attrs["allowed_event_types"] = attributes["allowedEventTypes"]
      attrs["transformation"] = attributes["transformation"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["transformation_updated_at"] = DateTime.rfc3339(attributes["transformationUpdatedAt"]).to_time
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["product_type"] = Svix::ConnectorProduct.deserialize(attributes["productType"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["orgId"] = Svix::serialize_primitive(@org_id) unless @org_id.nil?
      out["uid"] = Svix::serialize_primitive(@uid) unless @uid.nil?
      out["kind"] = Svix::serialize_schema_ref(@kind) unless @kind.nil?
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["logo"] = Svix::serialize_primitive(@logo) unless @logo.nil?
      out["description"] = Svix::serialize_primitive(@description) unless @description.nil?
      out["instructions"] = Svix::serialize_primitive(@instructions) unless @instructions.nil?
      out["allowedEventTypes"] = Svix::serialize_primitive(@allowed_event_types) unless @allowed_event_types.nil?
      out["transformation"] = Svix::serialize_primitive(@transformation) unless @transformation.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) unless @updated_at.nil?
      unless @transformation_updated_at.nil?
        out["transformationUpdatedAt"] = Svix::serialize_primitive(@transformation_updated_at)
      end

      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) unless @feature_flags.nil?
      out["productType"] = Svix::serialize_schema_ref(@product_type) unless @product_type.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
