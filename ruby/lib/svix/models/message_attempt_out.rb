# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageAttemptOut
    attr_accessor :url
    attr_accessor :response
    attr_accessor :response_status_code
    # Response duration in milliseconds.
    attr_accessor :response_duration_ms
    attr_accessor :status
    attr_accessor :status_text
    attr_accessor :trigger_type
    # The Message's ID.
    attr_accessor :msg_id
    # The Endpoint's ID.
    attr_accessor :endpoint_id
    # The MessageAttempt's ID.
    attr_accessor :id
    attr_accessor :timestamp
    attr_accessor :msg

    ALL_FIELD ||= [
      "url",
      "response",
      "response_status_code",
      "response_duration_ms",
      "status",
      "status_text",
      "trigger_type",
      "msg_id",
      "endpoint_id",
      "id",
      "timestamp",
      "msg"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageAttemptOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageAttemptOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["url"] = attributes["url"]
      attrs["response"] = attributes["response"]
      attrs["response_status_code"] = attributes["responseStatusCode"]
      attrs["response_duration_ms"] = attributes["responseDurationMs"]
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"])
      attrs["status_text"] = Svix::MessageStatusText.deserialize(attributes["statusText"])
      attrs["trigger_type"] = Svix::MessageAttemptTriggerType.deserialize(attributes["triggerType"])
      attrs["msg_id"] = attributes["msgId"]
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["id"] = attributes["id"]
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      attrs["msg"] = Svix::MessageOut.deserialize(attributes["msg"]) if attributes["msg"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["url"] = Svix::serialize_primitive(@url) unless @url.nil?
      out["response"] = Svix::serialize_primitive(@response) unless @response.nil?
      out["responseStatusCode"] = Svix::serialize_primitive(@response_status_code) unless @response_status_code.nil?
      out["responseDurationMs"] = Svix::serialize_primitive(@response_duration_ms) unless @response_duration_ms.nil?
      out["status"] = Svix::serialize_schema_ref(@status) unless @status.nil?
      out["statusText"] = Svix::serialize_schema_ref(@status_text) unless @status_text.nil?
      out["triggerType"] = Svix::serialize_schema_ref(@trigger_type) unless @trigger_type.nil?
      out["msgId"] = Svix::serialize_primitive(@msg_id) unless @msg_id.nil?
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) unless @endpoint_id.nil?
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["timestamp"] = Svix::serialize_primitive(@timestamp) unless @timestamp.nil?
      out["msg"] = Svix::serialize_schema_ref(@msg) unless @msg.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
