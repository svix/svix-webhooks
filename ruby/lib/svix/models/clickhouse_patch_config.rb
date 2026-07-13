# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ClickhousePatchConfig
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
      attrs["url"] = attributes["url"]
      attrs["username"] = attributes["username"]
      attrs["password"] = attributes["password"]
      attrs["database"] = attributes["database"]
      attrs["table_name"] = attributes["tableName"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["username"] = Svix::serialize_primitive(@username) if @username
      out["password"] = Svix::serialize_primitive(@password) if @password
      out["database"] = Svix::serialize_primitive(@database) if @database
      out["tableName"] = Svix::serialize_primitive(@table_name) if @table_name
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
