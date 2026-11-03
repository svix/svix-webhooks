# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class McpEventsUnsubscribeIn
    # The event the subscription was made for.
    attr_accessor :name
    # The filter arguments the subscription was made with, as given to
    # `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
    # in OpenAI's MCP Events docs.
    attr_accessor :arguments
    attr_accessor :delivery

    ALL_FIELD ||= ["name", "arguments", "delivery"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::McpEventsUnsubscribeIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::McpEventsUnsubscribeIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["name"] = attributes["name"]
      attrs["arguments"] = attributes["arguments"]
      attrs["delivery"] = Svix::McpEventsUnsubscribeDeliveryIn.deserialize(attributes["delivery"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["arguments"] = Svix::serialize_primitive(@arguments) unless @arguments.nil?
      out["delivery"] = Svix::serialize_schema_ref(@delivery) unless @delivery.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
