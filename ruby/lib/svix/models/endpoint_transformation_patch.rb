# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointTransformationPatch
    attr_accessor :code
    attr_accessor :enabled
    attr_accessor :variables

    ALL_FIELD ||= ["code", "enabled", "variables"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointTransformationPatch` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointTransformationPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["code"] = attributes["code"]
      attrs["enabled"] = attributes["enabled"]
      attrs["variables"] = attributes["variables"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["code"] = Svix::serialize_primitive(@code) if @__code_is_defined
      out["enabled"] = Svix::serialize_primitive(@enabled) if @enabled
      out["variables"] = Svix::serialize_primitive(@variables) if @__variables_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
