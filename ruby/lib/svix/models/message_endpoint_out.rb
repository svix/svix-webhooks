# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class MessageEndpointOut
    # The Endpoint's ID.
    attr_accessor :id
    attr_accessor :status
    attr_accessor :status_text
    attr_accessor :next_attempt
    # An example endpoint name.
    attr_accessor :description
    # Maximum messages per second to send to this endpoint.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # Optional unique identifier for the endpoint.
    attr_accessor :uid
    attr_accessor :url
    attr_accessor :disabled
    attr_accessor :filter_types
    # List of message channels this endpoint listens to (omit for all).
    attr_accessor :channels
    attr_accessor :created_at
    attr_accessor :updated_at

    ALL_FIELD ||= [
      "id",
      "status",
      "status_text",
      "next_attempt",
      "description",
      "throttle_rate",
      "uid",
      "url",
      "disabled",
      "filter_types",
      "channels",
      "created_at",
      "updated_at"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::MessageEndpointOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::MessageEndpointOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["status"] = Svix::MessageStatus.deserialize(attributes["status"])
      attrs["status_text"] = Svix::MessageStatusText.deserialize(attributes["statusText"])
      attrs["next_attempt"] = DateTime.rfc3339(attributes["nextAttempt"]).to_time if attributes["nextAttempt"]
      attrs["description"] = attributes["description"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["url"] = attributes["url"]
      attrs["disabled"] = attributes["disabled"]
      attrs["filter_types"] = attributes["filterTypes"]
      attrs["channels"] = attributes["channels"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["status"] = Svix::serialize_schema_ref(@status) if @status
      out["statusText"] = Svix::serialize_schema_ref(@status_text) if @status_text
      out["nextAttempt"] = Svix::serialize_primitive(@next_attempt) if @next_attempt
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["filterTypes"] = Svix::serialize_primitive(@filter_types) if @filter_types
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
