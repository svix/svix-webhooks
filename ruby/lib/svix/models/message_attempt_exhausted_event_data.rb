# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event.
  class MessageAttemptExhaustedEventData
    attr_accessor :app_id
    attr_accessor :app_uid
    attr_accessor :endpoint_id
    attr_accessor :last_attempt
    attr_accessor :msg_event_id
    attr_accessor :msg_id

    ALL_FIELD ||= ["app_id", "app_uid", "endpoint_id", "last_attempt", "msg_event_id", "msg_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::MessageAttemptExhaustedEventData` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageAttemptExhaustedEventData")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["app_id"] = attributes["appId"]
      attrs["app_uid"] = attributes["appUid"]
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["last_attempt"] = Svix::MessageAttemptFailedData.deserialize(attributes["lastAttempt"])
      attrs["msg_event_id"] = attributes["msgEventId"]
      attrs["msg_id"] = attributes["msgId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["appId"] = Svix::serialize_primitive(@app_id) if @app_id
      out["appUid"] = Svix::serialize_primitive(@app_uid) if @app_uid
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) if @endpoint_id
      out["lastAttempt"] = Svix::serialize_schema_ref(@last_attempt) if @last_attempt
      out["msgEventId"] = Svix::serialize_primitive(@msg_event_id) if @msg_event_id
      out["msgId"] = Svix::serialize_primitive(@msg_id) if @msg_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
