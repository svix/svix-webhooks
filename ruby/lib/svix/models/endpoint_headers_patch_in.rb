# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointHeadersPatchIn
    # A list of headers be be removed
    attr_accessor :delete_headers
    attr_accessor :headers

    ALL_FIELD ||= ["delete_headers", "headers"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointHeadersPatchIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointHeadersPatchIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["delete_headers"] = attributes["deleteHeaders"]
      attrs["headers"] = attributes["headers"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["deleteHeaders"] = Svix::serialize_primitive(@delete_headers) if @delete_headers
      out["headers"] = Svix::serialize_primitive(@headers) if @headers
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
