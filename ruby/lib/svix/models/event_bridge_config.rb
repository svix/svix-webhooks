# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventBridgeConfig
    attr_accessor :access_key_id
    # Free-form string, with a maximum of 128 characters
    attr_accessor :detail_type
    # The name or ARN of the event bus to receive the event
    attr_accessor :event_bus_name
    attr_accessor :region
    attr_accessor :secret_access_key

    ALL_FIELD ||= ["access_key_id", "detail_type", "event_bus_name", "region", "secret_access_key"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EventBridgeConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventBridgeConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["detail_type"] = attributes["detailType"]
      attrs["event_bus_name"] = attributes["eventBusName"]
      attrs["region"] = attributes["region"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) if @access_key_id
      out["detailType"] = Svix::serialize_primitive(@detail_type) if @detail_type
      out["eventBusName"] = Svix::serialize_primitive(@event_bus_name) if @event_bus_name
      out["region"] = Svix::serialize_primitive(@region) if @region
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) if @secret_access_key
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
