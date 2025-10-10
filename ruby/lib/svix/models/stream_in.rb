# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class StreamIn
    attr_accessor :metadata
    # The stream's name.
    attr_accessor :name
    # An optional unique identifier for the stream.
    attr_accessor :uid

    ALL_FIELD ||= ["metadata", "name", "uid"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::StreamIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["metadata"] = attributes["metadata"]
      attrs["name"] = attributes["name"]
      attrs["uid"] = attributes["uid"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
