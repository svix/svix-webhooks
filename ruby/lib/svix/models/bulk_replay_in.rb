# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class BulkReplayIn
    attr_accessor :channel
    attr_accessor :event_types
    attr_accessor :since
    attr_accessor :status
    attr_accessor :tag
    attr_accessor :until

    ALL_FIELD ||= ["channel", "event_types", "since", "status", "tag", "until"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::BulkReplayIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::BulkReplayIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["channel"] = attributes["channel"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["since"] = DateTime.rfc3339(attributes["since"]).to_time
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"]) if attributes["status"]
      attrs["tag"] = attributes["tag"]
      attrs["until"] = DateTime.rfc3339(attributes["until"]).to_time if attributes["until"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["channel"] = Svix::serialize_primitive(@channel) if @channel
      out["eventTypes"] = Svix::serialize_primitive(@event_types) if @event_types
      out["since"] = Svix::serialize_primitive(@since) if @since
      out["status"] = Svix::serialize_schema_ref(@status) if @status
      out["tag"] = Svix::serialize_primitive(@tag) if @tag
      out["until"] = Svix::serialize_primitive(@until) if @until
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
