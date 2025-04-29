# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class BackgroundTaskFinishedEvent2
    attr_accessor :data
    attr_accessor :status
    attr_accessor :task
    # The QueueBackgroundTask's ID.
    attr_accessor :task_id

    ALL_FIELD ||= ["data", "status", "task", "task_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::BackgroundTaskFinishedEvent2` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::BackgroundTaskFinishedEvent2")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["data"] = attributes["data"]
      attrs["status"] = Svix::BackgroundTaskStatus.deserialize(attributes["status"])
      attrs["task"] = Svix::BackgroundTaskType.deserialize(attributes["task"])
      attrs["task_id"] = attributes["taskId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["data"] = @data if @data
      out["status"] = Svix::serialize_schema_ref(@status) if @status
      out["task"] = Svix::serialize_schema_ref(@task) if @task
      out["taskId"] = Svix::serialize_primitive(@task_id) if @task_id
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
