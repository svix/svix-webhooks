# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApplicationIn
    attr_accessor :metadata
    # Application name for human consumption.
    attr_accessor :name
    # Deprecated, use `throttleRate` instead.
    attr_accessor :rate_limit
    # Maximum messages per second to send to this application.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # Optional unique identifier for the application.
    attr_accessor :uid

    ALL_FIELD ||= ["metadata", "name", "rate_limit", "throttle_rate", "uid"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ApplicationIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApplicationIn")
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
      attrs["rate_limit"] = attributes["rateLimit"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["rateLimit"] = Svix::serialize_primitive(@rate_limit) if @rate_limit
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
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
