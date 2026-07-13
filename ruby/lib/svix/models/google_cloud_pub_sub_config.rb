# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class GoogleCloudPubSubConfig
    attr_accessor :project_id
    attr_accessor :topic_id
    # Google Cloud Credentials JSON Object as a string.
    attr_accessor :credentials

    ALL_FIELD ||= ["project_id", "topic_id", "credentials"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::GoogleCloudPubSubConfig` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::GoogleCloudPubSubConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["project_id"] = attributes["projectId"]
      attrs["topic_id"] = attributes["topicId"]
      attrs["credentials"] = attributes["credentials"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["projectId"] = Svix::serialize_primitive(@project_id) if @project_id
      out["topicId"] = Svix::serialize_primitive(@topic_id) if @topic_id
      out["credentials"] = Svix::serialize_primitive(@credentials) if @credentials
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
