# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class CronConfig
    attr_accessor :schedule
    attr_accessor :payload
    # Override the default content-type.
    #
    # Recommended if the payload is not JSON.
    attr_accessor :content_type

    ALL_FIELD ||= ["schedule", "payload", "content_type"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::CronConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::CronConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["schedule"] = attributes["schedule"]
      attrs["payload"] = attributes["payload"]
      attrs["content_type"] = attributes["contentType"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["schedule"] = Svix::serialize_primitive(@schedule) if @schedule
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["contentType"] = Svix::serialize_primitive(@content_type) if @content_type
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
