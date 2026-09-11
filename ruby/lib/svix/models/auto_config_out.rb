# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AutoConfigOut
    attr_accessor :created_at
    attr_accessor :token
    # The AutoConfigSubscription's ID.
    attr_accessor :id

    ALL_FIELD ||= ["created_at", "token", "id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::AutoConfigOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AutoConfigOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["token"] = attributes["token"]
      attrs["id"] = attributes["id"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["token"] = Svix::serialize_primitive(@token) unless @token.nil?
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
