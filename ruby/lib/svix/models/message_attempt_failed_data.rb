# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageAttemptFailedData
    attr_accessor :id
    attr_accessor :response_status_code
    attr_accessor :timestamp

    ALL_FIELD ||= ["id", "response_status_code", "timestamp"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::MessageAttemptFailedData` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageAttemptFailedData")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["response_status_code"] = attributes["responseStatusCode"]
      attrs["timestamp"] = DateTime.rfc3339(attributes["timestamp"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["responseStatusCode"] = Svix::serialize_primitive(@response_status_code) if @response_status_code
      out["timestamp"] = Svix::serialize_primitive(@timestamp) if @timestamp
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
