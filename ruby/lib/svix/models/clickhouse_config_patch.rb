# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ClickhouseConfigPatch
    attr_accessor :url
    attr_accessor :username
    attr_accessor :password
    attr_accessor :database
    attr_accessor :table_name

    ALL_FIELD ||= ["url", "username", "password", "database", "table_name"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ClickhouseConfigPatch` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ClickhouseConfigPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["url"] = attributes["url"]
      attrs["username"] = attributes["username"]
      attrs["password"] = attributes["password"]
      attrs["database"] = attributes["database"]
      attrs["table_name"] = attributes["tableName"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["url"] = Svix::serialize_primitive(@url) unless @url.nil?
      out["username"] = Svix::serialize_primitive(@username) unless @username.nil?
      out["password"] = Svix::serialize_primitive(@password) unless @password.nil?
      out["database"] = Svix::serialize_primitive(@database) unless @database.nil?
      out["tableName"] = Svix::serialize_primitive(@table_name) unless @table_name.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
