# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageIn
    # Optionally creates a new application alongside the message.
    #
    # If the application id or uid that is used in the path already exists, this argument is ignored.
    attr_accessor :application
    # List of free-form identifiers that endpoints can filter by
    attr_accessor :channels
    # Optional unique identifier for the message
    attr_accessor :event_id
    # The event type's name
    attr_accessor :event_type
    # JSON payload to send as the request body of the webhook.
    #
    # We also support sending non-JSON payloads. Please contact us for more information.
    attr_accessor :payload
    # Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`.
    attr_accessor :payload_retention_hours
    # Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`.
    attr_accessor :payload_retention_period
    # List of free-form tags that can be filtered by when listing messages
    attr_accessor :tags
    # Extra parameters to pass to Transformations (for future use)
    attr_accessor :transformations_params

    ALL_FIELD ||= [
      "application",
      "channels",
      "event_id",
      "event_type",
      "payload",
      "payload_retention_hours",
      "payload_retention_period",
      "tags",
      "transformations_params"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["application"] = Svix::ApplicationIn.deserialize(attributes["application"]) if attributes["application"]
      attrs["channels"] = attributes["channels"]
      attrs["event_id"] = attributes["eventId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["payload"] = attributes["payload"]
      attrs["payload_retention_hours"] = attributes["payloadRetentionHours"]
      attrs["payload_retention_period"] = attributes["payloadRetentionPeriod"]
      attrs["tags"] = attributes["tags"]
      attrs["transformations_params"] = attributes["transformationsParams"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["application"] = Svix::serialize_schema_ref(@application) if @application
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["eventId"] = Svix::serialize_primitive(@event_id) if @event_id
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["payload"] = Svix::serialize_primitive(@payload) if @payload
      out["payloadRetentionHours"] = Svix::serialize_primitive(@payload_retention_hours) if @payload_retention_hours
      out["payloadRetentionPeriod"] = Svix::serialize_primitive(@payload_retention_period) if @payload_retention_period
      out["tags"] = Svix::serialize_primitive(@tags) if @tags
      out["transformationsParams"] = Svix::serialize_primitive(@transformations_params) if @transformations_params
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
