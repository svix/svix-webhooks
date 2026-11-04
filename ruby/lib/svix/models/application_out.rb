# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApplicationOut
    # Optional unique identifier for the application.
    attr_accessor :uid
    # Application name for human consumption.
    attr_accessor :name
    # Maximum messages per second to send to this application.
    #
    # Outgoing messages will be throttled to this rate.
    attr_accessor :throttle_rate
    # The Application's ID.
    attr_accessor :id
    attr_accessor :created_at
    attr_accessor :updated_at
    attr_accessor :metadata

    ALL_FIELD ||= ["uid", "name", "throttle_rate", "id", "created_at", "updated_at", "metadata"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ApplicationOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApplicationOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["uid"] = attributes["uid"]
      attrs["name"] = attributes["name"]
      attrs["throttle_rate"] = attributes["throttleRate"]
      attrs["id"] = attributes["id"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["metadata"] = attributes["metadata"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["uid"] = Svix::serialize_primitive(@uid) unless @uid.nil?
      out["name"] = Svix::serialize_primitive(@name) unless @name.nil?
      out["throttleRate"] = Svix::serialize_primitive(@throttle_rate) unless @throttle_rate.nil?
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) unless @updated_at.nil?
      out["metadata"] = Svix::serialize_primitive(@metadata) unless @metadata.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
