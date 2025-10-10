# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class StreamPortalAccessIn
    # How long the token will be valid for, in seconds.
    #
    # Valid values are between 1 hour and 7 days. The default is 7 days.
    attr_accessor :expiry
    # The set of feature flags the created token will have access to.
    attr_accessor :feature_flags
    # An optional session ID to attach to the token.
    #
    # When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
    attr_accessor :session_id

    ALL_FIELD ||= ["expiry", "feature_flags", "session_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::StreamPortalAccessIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamPortalAccessIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["expiry"] = attributes["expiry"]
      attrs["feature_flags"] = attributes["featureFlags"]
      attrs["session_id"] = attributes["sessionId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["expiry"] = Svix::serialize_primitive(@expiry) if @expiry
      out["featureFlags"] = Svix::serialize_primitive(@feature_flags) if @feature_flags
      out["sessionId"] = Svix::serialize_primitive(@session_id) if @session_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
