# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ClickhouseConfig
    # The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
    attr_accessor :url
    # Username to access Clickhouse
    attr_accessor :username
    # Password to access Clickhouse
    attr_accessor :password
    # The Clickhouse database to connect to
    attr_accessor :database
    # The Clickhouse table to write to
    attr_accessor :table_name

    ALL_FIELD ||= ["url", "username", "password", "database", "table_name"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::ClickhouseConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ClickhouseConfig")
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
