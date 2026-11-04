# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class McpEventsSubscribeOut
    # The subscription's ID, which is also the UID of its endpoint.
    attr_accessor :id
    # The Endpoint's ID.
    attr_accessor :endpoint_id
    # When the subscription expires, or `null` if it never does.
    attr_accessor :refresh_before
    # Always `null`, since events can't be replayed.
    attr_accessor :cursor
    # Always `false`, since events can't be replayed.
    attr_accessor :truncated

    ALL_FIELD ||= ["id", "endpoint_id", "refresh_before", "cursor", "truncated"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::McpEventsSubscribeOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::McpEventsSubscribeOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["endpoint_id"] = attributes["endpointId"]
      attrs["refresh_before"] = DateTime.rfc3339(attributes["refreshBefore"]).to_time if attributes["refreshBefore"]
      attrs["cursor"] = attributes["cursor"]
      attrs["truncated"] = attributes["truncated"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["endpointId"] = Svix::serialize_primitive(@endpoint_id) unless @endpoint_id.nil?
      out["refreshBefore"] = Svix::serialize_primitive(@refresh_before) unless @refresh_before.nil?
      out["cursor"] = Svix::serialize_primitive(@cursor) unless @cursor.nil?
      out["truncated"] = Svix::serialize_primitive(@truncated) unless @truncated.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
