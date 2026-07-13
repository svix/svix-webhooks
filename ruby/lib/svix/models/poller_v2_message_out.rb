# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # The MessageOut equivalent of polling endpoint
  class PollerV2MessageOut
    attr_accessor :offset
    attr_accessor :headers
    # Optional unique identifier for the message
    attr_accessor :event_id
    # The event type's name
    attr_accessor :event_type
    attr_accessor :payload
    # List of free-form identifiers that endpoints can filter by
    attr_accessor :channels
    # The Message's ID.
    attr_accessor :id
    attr_accessor :timestamp
    attr_accessor :tags
    attr_accessor :deliver_at

    ALL_FIELD ||= [
      "offset",
      "headers",
      "event_id",
      "event_type",
      "payload",
      "channels",
      "id",
      "timestamp",
      "tags",
      "deliver_at"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::PollerV2MessageOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::PollerV2MessageOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["offset"] = attributes["offset"]
      attrs["headers"] = attributes["headers"]
      attrs["event_id"] = attributes["eventId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["payload"] = attributes["payload"]
      attrs["channels"] = attributes["channels"]
      attrs["id"] = attributes["id"]
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      attrs["tags"] = attributes["tags"]
      attrs["deliver_at"] = DateTime.rfc3339(attributes["deliverAt"]).to_time if attributes["deliverAt"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["offset"] = Svix::serialize_primitive(@offset) if @offset
      out["headers"] = Svix::serialize_primitive(@headers) if @headers
      out["eventId"] = Svix::serialize_primitive(@event_id) if @event_id
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["timestamp"] = Svix::serialize_primitive(@timestamp) if @timestamp
      out["tags"] = Svix::serialize_primitive(@tags) if @tags
      out["deliverAt"] = Svix::serialize_primitive(@deliver_at) if @deliver_at
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
