# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeIn
    # The event type's name
    attr_accessor :name
    attr_accessor :description
    attr_accessor :archived
    attr_accessor :deprecated
    # The schema for the event type for a specific version as a JSON schema.
    attr_accessor :schemas
    # The event type group's name
    attr_accessor :group_name
    attr_accessor :feature_flags

    ALL_FIELD ||= ["name", "description", "archived", "deprecated", "schemas", "group_name", "feature_flags"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventTypeIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypeIn")
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
      attrs["archived"] = attributes["archived"]
      attrs["deprecated"] = attributes["deprecated"]
      attrs["schemas"] = attributes["schemas"]
      attrs["group_name"] = attributes["groupName"]
      attrs["feature_flags"] = attributes["featureFlags"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["description"] = Svix::serialize_primitive(@description) unless @description.nil?
      out["archived"] = Svix::serialize_primitive(@archived) unless @archived.nil?
      out["deprecated"] = Svix::serialize_primitive(@deprecated) unless @deprecated.nil?
      out["schemas"] = Svix::serialize_primitive(@schemas) unless @schemas.nil?
      out["groupName"] = Svix::serialize_primitive(@group_name) unless @group_name.nil?
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) unless @feature_flags.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
