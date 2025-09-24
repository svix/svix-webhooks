# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call.
  class IngestEndpointDisabledEventData
    # The Endpoint's ID.
    attr_accessor :endpoint_id
    # The Endpoint's UID.
    attr_accessor :endpoint_uid
    attr_accessor :fail_since
    # The Source's ID.
    attr_accessor :source_id
    attr_accessor :trigger

    ALL_FIELD ||= ["endpoint_id", "endpoint_uid", "fail_since", "source_id", "trigger"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::IngestEndpointDisabledEventData` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IngestEndpointDisabledEventData")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["endpoint_uid"] = attributes["endpointUid"]
      attrs["fail_since"] = DateTime.rfc3339(attributes["failSince"]).to_time if attributes["failSince"]
      attrs["source_id"] = attributes["sourceId"]
      attrs["trigger"] = Svix::EndpointDisabledTrigger.deserialize(attributes["trigger"]) if attributes["trigger"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) if @endpoint_id
      out["endpointUid"] = Svix::serialize_primitive(@endpoint_uid) if @endpoint_uid
      out["failSince"] = Svix::serialize_primitive(@fail_since) if @fail_since
      out["sourceId"] = Svix::serialize_primitive(@source_id) if @source_id
      out["trigger"] = Svix::serialize_schema_ref(@trigger) if @trigger
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
