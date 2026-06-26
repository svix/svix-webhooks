# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration parameters for defining a Redshift sink.
  #
  # For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
  class RedshiftConfig
    attr_accessor :access_key_id
    # Required for provisioned clusters.
    attr_accessor :cluster_identifier
    # Database name.
    #
    # Only required if not using transformations.
    attr_accessor :db_name
    # Required for provisioned clusters.
    attr_accessor :db_user
    attr_accessor :region
    # Schema name.
    #
    # Only used if not using transformations.
    attr_accessor :schema_name
    attr_accessor :secret_access_key
    # Table name.
    #
    # Only required if not using transformations.
    attr_accessor :table_name
    # Required for Redshift Serverless.
    attr_accessor :workgroup_name

    ALL_FIELD ||= [
      "access_key_id",
      "cluster_identifier",
      "db_name",
      "db_user",
      "region",
      "schema_name",
      "secret_access_key",
      "table_name",
      "workgroup_name"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RedshiftConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RedshiftConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["cluster_identifier"] = attributes["clusterIdentifier"]
      attrs["db_name"] = attributes["dbName"]
      attrs["db_user"] = attributes["dbUser"]
      attrs["region"] = attributes["region"]
      attrs["schema_name"] = attributes["schemaName"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["table_name"] = attributes["tableName"]
      attrs["workgroup_name"] = attributes["workgroupName"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) if @access_key_id
      out["clusterIdentifier"] = Svix::serialize_primitive(@cluster_identifier) if @cluster_identifier
      out["dbName"] = Svix::serialize_primitive(@db_name) if @db_name
      out["dbUser"] = Svix::serialize_primitive(@db_user) if @db_user
      out["region"] = Svix::serialize_primitive(@region) if @region
      out["schemaName"] = Svix::serialize_primitive(@schema_name) if @schema_name
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) if @secret_access_key
      out["tableName"] = Svix::serialize_primitive(@table_name) if @table_name
      out["workgroupName"] = Svix::serialize_primitive(@workgroup_name) if @workgroup_name
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
