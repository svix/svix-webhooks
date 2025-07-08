# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApplicationTokenExpireIn
    # How many seconds until the old key is expired.
    attr_accessor :expiry
    # An optional list of session ids.
    #
    # If any session ids are specified, only Application tokens created with that session id will be expired.
    attr_accessor :session_ids

    ALL_FIELD ||= ["expiry", "session_ids"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ApplicationTokenExpireIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApplicationTokenExpireIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["expiry"] = attributes["expiry"]
      attrs["session_ids"] = attributes["sessionIds"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["expiry"] = Svix::serialize_primitive(@expiry) if @expiry
      out["sessionIds"] = Svix::serialize_primitive(@session_ids) if @session_ids
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
