# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration for a Google Cloud BigQuery sink.
  class BigQueryConfig
    # Google Cloud Credentials JSON Object as a string.
    attr_accessor :credentials
    attr_accessor :dataset_id
    attr_accessor :project_id
    attr_accessor :table_id

    ALL_FIELD ||= ["credentials", "dataset_id", "project_id", "table_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::BigQueryConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::BigQueryConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["credentials"] = attributes["credentials"]
      attrs["dataset_id"] = attributes["datasetId"]
      attrs["project_id"] = attributes["projectId"]
      attrs["table_id"] = attributes["tableId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["credentials"] = Svix::serialize_primitive(@credentials) if @credentials
      out["datasetId"] = Svix::serialize_primitive(@dataset_id) if @dataset_id
      out["projectId"] = Svix::serialize_primitive(@project_id) if @project_id
      out["tableId"] = Svix::serialize_primitive(@table_id) if @table_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
