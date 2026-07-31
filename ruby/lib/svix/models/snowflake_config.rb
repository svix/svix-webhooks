# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration parameters for defining a Snowflake sink.
  class SnowflakeConfig
    # PEM-encoded private key used for signing token-based requests to the Snowflake API.
    #
    # Beginning/end delimiters are not required.
    attr_accessor :private_key
    # Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen.
    attr_accessor :account_identifier
    # The Snowflake user id.
    attr_accessor :user_id
    # Database name.
    #
    # Only required if not using transformations.
    attr_accessor :db_name
    # Schema name.
    #
    # Only required if not using transformations.
    attr_accessor :schema_name
    # Table name.
    #
    # Only required if not using transformations.
    attr_accessor :table_name

    ALL_FIELD ||= ["private_key", "account_identifier", "user_id", "db_name", "schema_name", "table_name"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SnowflakeConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SnowflakeConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["private_key"] = attributes["privateKey"]
      attrs["account_identifier"] = attributes["accountIdentifier"]
      attrs["user_id"] = attributes["userId"]
      attrs["db_name"] = attributes["dbName"]
      attrs["schema_name"] = attributes["schemaName"]
      attrs["table_name"] = attributes["tableName"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["privateKey"] = Svix::serialize_primitive(@private_key) if @private_key
      out["accountIdentifier"] = Svix::serialize_primitive(@account_identifier) if @account_identifier
      out["userId"] = Svix::serialize_primitive(@user_id) if @user_id
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
