# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageIn
    # Optional unique identifier for the message
    attr_accessor :event_id
    # The event type's name
    attr_accessor :event_type
    # JSON payload to send as the request body of the webhook.
    #
    # We also support sending non-JSON payloads. Please contact us for more information.
    attr_accessor :payload
    # List of free-form identifiers that endpoints can filter by
    attr_accessor :channels
    # Optionally creates a new application alongside the message.
    #
    # If the application id or uid that is used in the path already exists, this argument is ignored.
    attr_accessor :application
    # List of free-form tags that can be filtered by when listing messages
    attr_accessor :tags
    # Extra parameters to pass to Transformations (for future use)
    attr_accessor :transformations_params
    # The date and time at which the message will be delivered.
    #
    # Note that this time is best-effort-only. Must be at least one minute and no more than 24 hours in the future.
    attr_accessor :deliver_at
    # Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`.
    attr_accessor :payload_retention_period
    # Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`.
    attr_accessor :payload_retention_hours

    ALL_FIELD ||= [
      "event_id",
      "event_type",
      "payload",
      "channels",
      "application",
      "tags",
      "transformations_params",
      "deliver_at",
      "payload_retention_period",
      "payload_retention_hours"
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
      attrs["event_id"] = attributes["eventId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["payload"] = attributes["payload"]
      attrs["channels"] = attributes["channels"]
      attrs["application"] = Svix::ApplicationIn.deserialize(attributes["application"]) if attributes["application"]
      attrs["tags"] = attributes["tags"]
      attrs["transformations_params"] = attributes["transformationsParams"]
      attrs["deliver_at"] = DateTime.rfc3339(attributes["deliverAt"]).to_time if attributes["deliverAt"]
      attrs["payload_retention_period"] = attributes["payloadRetentionPeriod"]
      attrs["payload_retention_hours"] = attributes["payloadRetentionHours"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["eventId"] = Svix::serialize_primitive(@event_id) unless @event_id.nil?
      out["eventType"] = Svix::serialize_primitive(@event_type) unless @event_type.nil?
      out["payload"] = Svix::serialize_primitive(@payload) unless @payload.nil?
      out["channels"] = Svix::serialize_primitive(@channels) unless @channels.nil?
      out["application"] = Svix::serialize_schema_ref(@application) unless @application.nil?
      out["tags"] = Svix::serialize_primitive(@tags) unless @tags.nil?
      unless @transformations_params.nil?
        out["transformationsParams"] = Svix::serialize_primitive(@transformations_params)
      end

      out["deliverAt"] = Svix::serialize_primitive(@deliver_at) unless @deliver_at.nil?
      unless @payload_retention_period.nil?
        out["payloadRetentionPeriod"] = Svix::serialize_primitive(@payload_retention_period)
      end

      unless @payload_retention_hours.nil?
        out["payloadRetentionHours"] = Svix::serialize_primitive(@payload_retention_hours)
      end

      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
