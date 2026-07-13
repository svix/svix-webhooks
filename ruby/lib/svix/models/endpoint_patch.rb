# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointPatch
    attr_accessor :description
    # Maximum messages per second to send to this endpoint.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # The Endpoint's UID.
    attr_accessor :uid
    attr_accessor :url
    attr_accessor :disabled
    attr_accessor :filter_types
    attr_accessor :channels
    attr_accessor :metadata

    ALL_FIELD ||= ["description", "throttle_rate", "uid", "url", "disabled", "filter_types", "channels", "metadata"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["description"] = attributes["description"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["url"] = attributes["url"]
      attrs["disabled"] = attributes["disabled"]
      attrs["filter_types"] = attributes["filterTypes"]
      attrs["channels"] = attributes["channels"]
      attrs["metadata"] = attributes["metadata"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @__throttle_rate_is_defined
      out["uid"] = Svix::serialize_primitive(@uid) if @__uid_is_defined
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["filterTypes"] = Svix::serialize_primitive(@filter_types) if @__filter_types_is_defined
      out["channels"] = Svix::serialize_primitive(@channels) if @__channels_is_defined
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
