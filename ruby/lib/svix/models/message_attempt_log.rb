# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageAttemptLog
    # The Application's ID.
    attr_accessor :app_id
    # The Application's UID.
    attr_accessor :app_uid
    attr_accessor :attempt_count
    attr_accessor :attempt_end
    # The MessageAttempt's ID.
    attr_accessor :attempt_id
    attr_accessor :attempt_start
    # The Endpoint's ID.
    attr_accessor :endpoint_id
    # The event type's name
    attr_accessor :event_type
    attr_accessor :http_times
    attr_accessor :msg_created
    # The Message's UID.
    attr_accessor :msg_event_id
    # The Message's ID.
    attr_accessor :msg_id
    # The Environment's ID.
    attr_accessor :org_id
    attr_accessor :response_status_code
    attr_accessor :status

    ALL_FIELD ||= [
      "app_id",
      "app_uid",
      "attempt_count",
      "attempt_end",
      "attempt_id",
      "attempt_start",
      "endpoint_id",
      "event_type",
      "http_times",
      "msg_created",
      "msg_event_id",
      "msg_id",
      "org_id",
      "response_status_code",
      "status"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageAttemptLog` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageAttemptLog")
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
      attrs["attempt_count"] = attributes["attemptCount"]
      attrs["attempt_end"] = DateTime.rfc3339(attributes["attemptEnd"]).to_time
      attrs["attempt_id"] = attributes["attemptId"]
      attrs["attempt_start"] = DateTime.rfc3339(attributes["attemptStart"]).to_time
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["event_type"] = attributes["eventType"]
      attrs["http_times"] = Svix::HttpAttemptTimes.deserialize(attributes["httpTimes"]) if attributes["httpTimes"]
      attrs["msg_created"] = DateTime.rfc3339(attributes["msgCreated"]).to_time
      attrs["msg_event_id"] = attributes["msgEventId"]
      attrs["msg_id"] = attributes["msgId"]
      attrs["org_id"] = attributes["orgId"]
      attrs["response_status_code"] = attributes["responseStatusCode"]
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["appId"] = Svix::serialize_primitive(@app_id) if @app_id
      out["appUid"] = Svix::serialize_primitive(@app_uid) if @app_uid
      out["attemptCount"] = Svix::serialize_primitive(@attempt_count) if @attempt_count
      out["attemptEnd"] = Svix::serialize_primitive(@attempt_end) if @attempt_end
      out["attemptId"] = Svix::serialize_primitive(@attempt_id) if @attempt_id
      out["attemptStart"] = Svix::serialize_primitive(@attempt_start) if @attempt_start
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) if @endpoint_id
      out["eventType"] = Svix::serialize_primitive(@event_type) if @event_type
      out["httpTimes"] = Svix::serialize_schema_ref(@http_times) if @http_times
      out["msgCreated"] = Svix::serialize_primitive(@msg_created) if @msg_created
      out["msgEventId"] = Svix::serialize_primitive(@msg_event_id) if @msg_event_id
      out["msgId"] = Svix::serialize_primitive(@msg_id) if @msg_id
      out["orgId"] = Svix::serialize_primitive(@org_id) if @org_id
      out["responseStatusCode"] = Svix::serialize_primitive(@response_status_code) if @response_status_code
      out["status"] = Svix::serialize_schema_ref(@status) if @status
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
