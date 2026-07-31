# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RedshiftPatchConfig
    attr_accessor :access_key_id
    attr_accessor :secret_access_key
    attr_accessor :region
    # Database name.
    #
    # Only required if not using transformations.
    attr_accessor :db_name
    # Schema name.
    #
    # Only used if not using transformations.
    attr_accessor :schema_name
    # Table name.
    #
    # Only required if not using transformations.
    attr_accessor :table_name

    ALL_FIELD ||= ["access_key_id", "secret_access_key", "region", "db_name", "schema_name", "table_name"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RedshiftPatchConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RedshiftPatchConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["region"] = attributes["region"]
      attrs["db_name"] = attributes["dbName"]
      attrs["schema_name"] = attributes["schemaName"]
      attrs["table_name"] = attributes["tableName"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) if @access_key_id
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) if @secret_access_key
      out["region"] = Svix::serialize_primitive(@region) if @region
      out["dbName"] = Svix::serialize_primitive(@db_name) if @db_name
      out["schemaName"] = Svix::serialize_primitive(@schema_name) if @schema_name
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
