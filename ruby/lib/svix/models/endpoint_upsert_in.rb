# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointUpsertIn
    attr_accessor :url
    attr_accessor :description
    # Maximum messages per second to send to this endpoint.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # Optional unique identifier for the endpoint.
    attr_accessor :uid
    attr_accessor :disabled
    attr_accessor :event_types
    # List of message channels this endpoint listens to (omit for all).
    attr_accessor :channels
    attr_accessor :metadata

    ALL_FIELD ||= ["url", "description", "throttle_rate", "uid", "disabled", "event_types", "channels", "metadata"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointUpsertIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointUpsertIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["url"] = attributes["url"]
      attrs["description"] = attributes["description"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["disabled"] = attributes["disabled"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["channels"] = attributes["channels"]
      attrs["metadata"] = attributes["metadata"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["eventTypes"] = Svix::serialize_primitive(@event_types) if @event_types
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
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
