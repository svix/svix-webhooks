# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class McpEventsSubscribeIn
    # The event to subscribe to. It must be an existing event type.
    attr_accessor :name
    # The filter arguments of the `events/subscribe` request, as described by
    # the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
    # Together with `name` and the callback URL they identify the subscription
    #
    # They're stored in the endpoint's `mcpEventArguments` metadata, but
    # aren't used to filter messages; use `channels` for that.
    #
    # See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
    # in OpenAI's MCP Events docs.
    attr_accessor :arguments
    attr_accessor :delivery
    # The requested lifetime of the subscription, in milliseconds. Omit for
    # the default of 7 days, or set to `null` for a subscription that never
    # expires.
    attr_accessor :ttl_ms
    # How long the previous secret keeps signing when a refresh changes it,
    # in seconds.
    #
    # Valid values are between 0 (immediate expiry) and 7 days. The default
    # is 5 minutes.
    attr_accessor :secret_grace_period_seconds
    # How the callback must respond to the verification challenge.
    attr_accessor :challenge_mode
    # Only send messages with one of these channels to the subscription,
    # for example ones derived from `arguments`. Omit to send all messages
    # of the event type.
    attr_accessor :channels

    ALL_FIELD ||= [
      "name",
      "arguments",
      "delivery",
      "ttl_ms",
      "secret_grace_period_seconds",
      "challenge_mode",
      "channels"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::McpEventsSubscribeIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::McpEventsSubscribeIn")
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
      attrs["delivery"] = Svix::McpEventsSubscribeDeliveryIn.deserialize(attributes["delivery"])
      attrs["ttl_ms"] = attributes["ttlMs"]
      attrs["secret_grace_period_seconds"] = attributes["secretGracePeriodSeconds"]
      if attributes["challengeMode"]
        attrs["challenge_mode"] = Svix::EndpointChallengeMode.deserialize(attributes["challengeMode"])
      end

      attrs["channels"] = attributes["channels"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["arguments"] = Svix::serialize_primitive(@arguments) unless @arguments.nil?
      out["delivery"] = Svix::serialize_schema_ref(@delivery) unless @delivery.nil?
      out["ttlMs"] = Svix::serialize_primitive(@ttl_ms) unless @ttl_ms.nil?
      unless @secret_grace_period_seconds.nil?
        out["secretGracePeriodSeconds"] = Svix::serialize_primitive(@secret_grace_period_seconds)
      end

      out["challengeMode"] = Svix::serialize_schema_ref(@challenge_mode) unless @challenge_mode.nil?
      out["channels"] = Svix::serialize_primitive(@channels) unless @channels.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
