# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointTransformationOut
    attr_accessor :code
    attr_accessor :enabled
    attr_accessor :updated_at

    ALL_FIELD ||= ["code", "enabled", "updated_at"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointTransformationOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointTransformationOut")
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
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time if attributes["updatedAt"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["code"] = Svix::serialize_primitive(@code) if @code
      out["enabled"] = Svix::serialize_primitive(@enabled) if @enabled
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
