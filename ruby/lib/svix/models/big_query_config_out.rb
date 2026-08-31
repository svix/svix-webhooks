# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class BigQueryConfigOut
    attr_accessor :project_id
    attr_accessor :dataset_id
    attr_accessor :table_id

    ALL_FIELD ||= ["project_id", "dataset_id", "table_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::BigQueryConfigOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::BigQueryConfigOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["project_id"] = attributes["projectId"]
      attrs["dataset_id"] = attributes["datasetId"]
      attrs["table_id"] = attributes["tableId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["projectId"] = Svix::serialize_primitive(@project_id) if @project_id
      out["datasetId"] = Svix::serialize_primitive(@dataset_id) if @dataset_id
      out["tableId"] = Svix::serialize_primitive(@table_id) if @table_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
