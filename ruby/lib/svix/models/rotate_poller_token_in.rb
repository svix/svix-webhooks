# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RotatePollerTokenIn
    # How long the token will be valid for, in seconds. Can be up to 31,536,000 seconds (1 year).
    attr_accessor :expiry
    # Updates the previous token's expiration, in seconds.
    #
    # If set to 0, the old token will immediately be revoked. Must be between 0 and 86,400 seconds (1 day).
    #
    # Defaults to 300 seconds (5 minutes).
    attr_accessor :old_token_expiry

    ALL_FIELD ||= ["expiry", "old_token_expiry"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RotatePollerTokenIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RotatePollerTokenIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["expiry"] = attributes["expiry"]
      attrs["old_token_expiry"] = attributes["oldTokenExpiry"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["expiry"] = Svix::serialize_primitive(@expiry) if @expiry
      out["oldTokenExpiry"] = Svix::serialize_primitive(@old_token_expiry) if @old_token_expiry
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
