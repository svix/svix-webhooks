# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class GenericConfig
    attr_accessor :response

    ALL_FIELD ||= ["response"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::GenericConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::GenericConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["response"] = Svix::GenericResponse.deserialize(attributes["response"]) if attributes["response"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["response"] = Svix::serialize_schema_ref(@response) unless @response.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
