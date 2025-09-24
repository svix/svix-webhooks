# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Sent after a message has been failing for a few times.
  # It's sent on the fourth failure. It complements `ingest.message.attempt.exhausted` which is sent after the last failure.
  class IngestMessageAttemptFailingEvent
    attr_accessor :data
    attr_accessor :type

    ALL_FIELD ||= ["data", "type"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::IngestMessageAttemptFailingEvent` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IngestMessageAttemptFailingEvent")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["data"] = Svix::IngestMessageAttemptFailingEventData.deserialize(attributes["data"])
      attrs["type"] = attributes["type"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["data"] = Svix::serialize_schema_ref(@data) if @data
      out["type"] = Svix::serialize_primitive(@type) if @type
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
