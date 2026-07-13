# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EventBridgePatchConfig
    attr_accessor :event_bus_name
    attr_accessor :detail_type
    attr_accessor :access_key_id
    attr_accessor :secret_access_key
    attr_accessor :region

    ALL_FIELD ||= ["event_bus_name", "detail_type", "access_key_id", "secret_access_key", "region"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::EventBridgePatchConfig` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EventBridgePatchConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["event_bus_name"] = attributes["eventBusName"]
      attrs["detail_type"] = attributes["detailType"]
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["region"] = attributes["region"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["eventBusName"] = Svix::serialize_primitive(@event_bus_name) if @event_bus_name
      out["detailType"] = Svix::serialize_primitive(@detail_type) if @detail_type
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) if @access_key_id
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) if @secret_access_key
      out["region"] = Svix::serialize_primitive(@region) if @region
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
