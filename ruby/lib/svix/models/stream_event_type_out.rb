# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class StreamEventTypeOut
    # The event type's name
    attr_accessor :name
    attr_accessor :description
    attr_accessor :created_at
    attr_accessor :updated_at
    attr_accessor :deprecated
    attr_accessor :archived
    attr_accessor :feature_flags

    ALL_FIELD ||= ["name", "description", "created_at", "updated_at", "deprecated", "archived", "feature_flags"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::StreamEventTypeOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamEventTypeOut")
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
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["deprecated"] = attributes["deprecated"]
      attrs["archived"] = attributes["archived"]
      attrs["feature_flags"] = attributes["featureFlags"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["description"] = Svix::serialize_primitive(@description) unless @description.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) unless @updated_at.nil?
      out["deprecated"] = Svix::serialize_primitive(@deprecated) unless @deprecated.nil?
      out["archived"] = Svix::serialize_primitive(@archived) unless @archived.nil?
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
