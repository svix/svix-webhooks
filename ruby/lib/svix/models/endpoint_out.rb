# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointOut
    # The Endpoint's ID.
    attr_accessor :id
    attr_accessor :metadata
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
    attr_accessor :created_at
    attr_accessor :updated_at

    ALL_FIELD ||= [
      "id",
      "metadata",
      "url",
      "description",
      "throttle_rate",
      "uid",
      "disabled",
      "event_types",
      "channels",
      "created_at",
      "updated_at"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["metadata"] = attributes["metadata"]
      attrs["url"] = attributes["url"]
      attrs["description"] = attributes["description"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["disabled"] = attributes["disabled"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["channels"] = attributes["channels"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["eventTypes"] = Svix::serialize_primitive(@event_types) if @event_types
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
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
