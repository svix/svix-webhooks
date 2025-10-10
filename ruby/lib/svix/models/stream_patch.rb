# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class StreamPatch
    # The Stream's description.
    attr_accessor :description
    attr_accessor :metadata
    # An optional unique identifier for the stream.
    attr_accessor :uid

    ALL_FIELD ||= ["description", "metadata", "uid"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::StreamPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["description"] = attributes["description"]
      attrs["metadata"] = attributes["metadata"]
      attrs["uid"] = attributes["uid"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["uid"] = Svix::serialize_primitive(@uid) if @__uid_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
