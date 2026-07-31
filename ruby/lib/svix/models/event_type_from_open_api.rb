# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeFromOpenApi
    # The event type's name
    attr_accessor :name
    attr_accessor :description
    attr_accessor :schemas
    attr_accessor :deprecated
    # The event type group's name
    attr_accessor :group_name
    attr_accessor :feature_flags

    ALL_FIELD ||= ["name", "description", "schemas", "deprecated", "group_name", "feature_flags"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventTypeFromOpenApi` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypeFromOpenApi")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["name"] = attributes["name"]
      attrs["description"] = attributes["description"]
      attrs["schemas"] = attributes["schemas"]
      attrs["deprecated"] = attributes["deprecated"]
      attrs["group_name"] = attributes["groupName"]
      attrs["feature_flags"] = attributes["featureFlags"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["schemas"] = Svix::serialize_primitive(@schemas) if @schemas
      out["deprecated"] = Svix::serialize_primitive(@deprecated) if @deprecated
      out["groupName"] = Svix::serialize_primitive(@group_name) if @group_name
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
