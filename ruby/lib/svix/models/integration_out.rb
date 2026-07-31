# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class IntegrationOut
    attr_accessor :name
    # The Integration's ID.
    attr_accessor :id
    attr_accessor :created_at
    attr_accessor :updated_at
    # The set of feature flags the integration has access to.
    attr_accessor :feature_flags

    ALL_FIELD ||= ["name", "id", "created_at", "updated_at", "feature_flags"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::IntegrationOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IntegrationOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["name"] = attributes["name"]
      attrs["id"] = attributes["id"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["feature_flags"] = attributes["featureFlags"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
