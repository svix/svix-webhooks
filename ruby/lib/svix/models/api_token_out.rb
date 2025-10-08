# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ApiTokenOut
    attr_accessor :created_at
    attr_accessor :expires_at
    # The GlobalApplicationToken's ID.
    attr_accessor :id
    attr_accessor :name
    attr_accessor :scopes
    attr_accessor :token

    ALL_FIELD ||= ["created_at", "expires_at", "id", "name", "scopes", "token"].freeze
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
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["expires_at"] = DateTime.rfc3339(attributes["expiresAt"]).to_time if attributes["expiresAt"]
      attrs["id"] = attributes["id"]
      attrs["name"] = attributes["name"]
      attrs["scopes"] = attributes["scopes"]
      attrs["token"] = attributes["token"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["expiresAt"] = Svix::serialize_primitive(@expires_at) if @expires_at
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["name"] = Svix::serialize_primitive(@name) if @name
      out["scopes"] = Svix::serialize_primitive(@scopes) if @scopes
      out["token"] = Svix::serialize_primitive(@token) if @token
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
