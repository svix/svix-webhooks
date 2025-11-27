# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class HttpAttemptTimes
    attr_accessor :end
    attr_accessor :start

    ALL_FIELD ||= ["end", "start"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::HttpAttemptTimes` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::HttpAttemptTimes")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["end"] = DateTime.rfc3339(attributes["end"]).to_time
      attrs["start"] = DateTime.rfc3339(attributes["start"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["end"] = Svix::serialize_primitive(@end) if @end
      out["start"] = Svix::serialize_primitive(@start) if @start
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
