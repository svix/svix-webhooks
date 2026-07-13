# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApiTokenOut
    attr_accessor :token
    attr_accessor :id
    attr_accessor :name
    attr_accessor :created_at
    attr_accessor :expires_at
    attr_accessor :scopes

    ALL_FIELD ||= ["token", "id", "name", "created_at", "expires_at", "scopes"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ApiTokenOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ApiTokenOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["token"] = attributes["token"]
      attrs["id"] = attributes["id"]
      attrs["name"] = attributes["name"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["expires_at"] = DateTime.rfc3339(attributes["expiresAt"]).to_time if attributes["expiresAt"]
      attrs["scopes"] = attributes["scopes"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["token"] = Svix::serialize_primitive(@token) if @token
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["expiresAt"] = Svix::serialize_primitive(@expires_at) if @expires_at
      out["scopes"] = Svix::serialize_primitive(@scopes) if @scopes
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
