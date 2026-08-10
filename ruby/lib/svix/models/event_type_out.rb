# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventTypeOut
    # The event type's name
    attr_accessor :name
    attr_accessor :description
    attr_accessor :archived
    attr_accessor :deprecated
    # The schema for the event type for a specific version as a JSON schema.
    attr_accessor :schemas
    attr_accessor :created_at
    attr_accessor :updated_at
    # The event type group's name
    attr_accessor :group_name
    attr_accessor :feature_flags
    attr_accessor :feature_flag

    ALL_FIELD ||= [
      "name",
      "description",
      "archived",
      "deprecated",
      "schemas",
      "created_at",
      "updated_at",
      "group_name",
      "feature_flags",
      "feature_flag"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventTypeOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventTypeOut")
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
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["group_name"] = attributes["groupName"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["feature_flag"] = attributes["featureFlag"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["description"] = Svix::serialize_primitive(@description) unless @description.nil?
      out["archived"] = Svix::serialize_primitive(@archived) unless @archived.nil?
      out["deprecated"] = Svix::serialize_primitive(@deprecated) unless @deprecated.nil?
      out["schemas"] = Svix::serialize_primitive(@schemas) unless @schemas.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) unless @updated_at.nil?
      out["groupName"] = Svix::serialize_primitive(@group_name) unless @group_name.nil?
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) unless @feature_flags.nil?
      out["featureFlag"] = Svix::serialize_primitive(@feature_flag) unless @feature_flag.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
