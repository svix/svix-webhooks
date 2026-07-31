# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class IngestEndpointOut
    # The Endpoint's ID.
    attr_accessor :id
    attr_accessor :url
    attr_accessor :description
    # Maximum messages per second to send to this endpoint.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # Optional unique identifier for the endpoint.
    attr_accessor :uid
    attr_accessor :disabled
    attr_accessor :created_at
    attr_accessor :updated_at
    attr_accessor :metadata

    ALL_FIELD ||= [
      "id",
      "url",
      "description",
      "throttle_rate",
      "uid",
      "disabled",
      "created_at",
      "updated_at",
      "metadata"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::IngestEndpointOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::IngestEndpointOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["id"] = attributes["id"]
      attrs["url"] = attributes["url"]
      attrs["description"] = attributes["description"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["uid"] = attributes["uid"]
      attrs["disabled"] = attributes["disabled"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["metadata"] = attributes["metadata"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) if @throttle_rate
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
