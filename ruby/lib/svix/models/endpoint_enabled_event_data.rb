# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Sent when an endpoint has been enabled.
  class EndpointEnabledEventData
    attr_accessor :app_id
    attr_accessor :app_uid
    attr_accessor :endpoint_id
    attr_accessor :endpoint_uid

    ALL_FIELD ||= ["app_id", "app_uid", "endpoint_id", "endpoint_uid"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EndpointEnabledEventData` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointEnabledEventData")
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
      attrs["endpoint_uid"] = attributes["endpointUid"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["appId"] = Svix::serialize_primitive(@app_id) if @app_id
      out["appUid"] = Svix::serialize_primitive(@app_uid) if @app_uid
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) if @endpoint_id
      out["endpointUid"] = Svix::serialize_primitive(@endpoint_uid) if @endpoint_uid
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
