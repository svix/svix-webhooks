# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class TailscaleConfig
    # Shared secret for Tailscale Webhooks
    attr_accessor :secret
    # Grace period (in seconds) for the timestamp.
    #
    # If not passed, timestamp age will not be checked.
    attr_accessor :timestamp_grace_seconds

    ALL_FIELD ||= ["secret", "timestamp_grace_seconds"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::TailscaleConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::TailscaleConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["secret"] = attributes["secret"]
      attrs["timestamp_grace_seconds"] = attributes["timestampGraceSeconds"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["secret"] = Svix::serialize_primitive(@secret) if @secret
      out["timestampGraceSeconds"] = Svix::serialize_primitive(@timestamp_grace_seconds) if @timestamp_grace_seconds
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
