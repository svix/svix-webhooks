# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class McpEventsSubscribeDeliveryIn
    # How events are delivered. Only `webhook` is supported.
    attr_accessor :mode
    # The callback URL events are sent to.
    attr_accessor :url
    # The secret events are signed with. MCP Events requires a `whsec_`
    # secret.
    attr_accessor :secret

    ALL_FIELD ||= ["mode", "url", "secret"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::McpEventsSubscribeDeliveryIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::McpEventsSubscribeDeliveryIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["mode"] = Svix::McpEventsDeliveryMode.deserialize(attributes["mode"])
      attrs["url"] = attributes["url"]
      attrs["secret"] = attributes["secret"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["mode"] = Svix::serialize_schema_ref(@mode) unless @mode.nil?
      out["url"] = Svix::serialize_primitive(@url) unless @url.nil?
      out["secret"] = Svix::serialize_primitive(@secret) unless @secret.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
