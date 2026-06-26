# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ClickhousePatchConfig
    attr_accessor :database
    attr_accessor :password
    attr_accessor :table_name
    attr_accessor :url
    attr_accessor :username

    ALL_FIELD ||= ["database", "password", "table_name", "url", "username"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ClickhousePatchConfig` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ClickhousePatchConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["database"] = attributes["database"]
      attrs["password"] = attributes["password"]
      attrs["table_name"] = attributes["tableName"]
      attrs["url"] = attributes["url"]
      attrs["username"] = attributes["username"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["database"] = Svix::serialize_primitive(@database) if @database
      out["password"] = Svix::serialize_primitive(@password) if @password
      out["tableName"] = Svix::serialize_primitive(@table_name) if @table_name
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["username"] = Svix::serialize_primitive(@username) if @username
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
