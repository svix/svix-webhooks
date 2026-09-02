# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class PostgresConfigPatch
    attr_accessor :url
    attr_accessor :password
    attr_accessor :table_name
    attr_accessor :ssl_root_cert

    ALL_FIELD ||= ["url", "password", "table_name", "ssl_root_cert"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::PostgresConfigPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::PostgresConfigPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["url"] = attributes["url"]
      attrs["password"] = attributes["password"]
      attrs["table_name"] = attributes["tableName"]
      attrs["ssl_root_cert"] = attributes["sslRootCert"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["url"] = Svix::serialize_primitive(@url) unless @url.nil?
      out["password"] = Svix::serialize_primitive(@password) unless @password.nil?
      out["tableName"] = Svix::serialize_primitive(@table_name) unless @table_name.nil?
      out["sslRootCert"] = Svix::serialize_primitive(@ssl_root_cert) unless @ssl_root_cert.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
