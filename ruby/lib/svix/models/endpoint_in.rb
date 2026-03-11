# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointIn
    # List of message channels this endpoint listens to (omit for all).
    attr_accessor :channels
    attr_accessor :description
    attr_accessor :disabled
    attr_accessor :filter_types
    attr_accessor :headers
    attr_accessor :metadata
    # Deprecated, use `throttleRate` instead.
    attr_accessor :rate_limit
    # The endpoint's verification secret.
    #
    # Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    # It is recommended to not set this and let the server generate the secret.
    attr_accessor :secret
    # Maximum messages per second to send to this endpoint.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # Optional unique identifier for the endpoint.
    attr_accessor :uid
    attr_accessor :url
    attr_accessor :version

    ALL_FIELD ||= [
      "channels",
      "description",
      "disabled",
      "filter_types",
      "headers",
      "metadata",
      "rate_limit",
      "secret",
      "throttle_rate",
      "uid",
      "url",
      "version"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["channels"] = attributes["channels"]
      attrs["description"] = attributes["description"]
      attrs["disabled"] = attributes["disabled"]
      attrs["filter_types"] = attributes["filterTypes"]
      attrs["headers"] = attributes["headers"]
      attrs["metadata"] = attributes["metadata"]
      attrs["rate_limit"] = attributes["rateLimit"]
      attrs["secret"] = attributes["secret"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["url"] = attributes["url"]
      attrs["version"] = attributes["version"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["filterTypes"] = Svix::serialize_primitive(@filter_types) if @filter_types
      out["headers"] = Svix::serialize_primitive(@headers) if @headers
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["rateLimit"] = Svix::serialize_primitive(@rate_limit) if @rate_limit
      out["secret"] = Svix::serialize_primitive(@secret) if @secret
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["version"] = Svix::serialize_primitive(@version) if @version
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
