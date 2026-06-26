# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration for a SNS sink.
  class SnsConfig
    attr_accessor :access_key_id
    attr_accessor :endpoint_url
    attr_accessor :region
    attr_accessor :secret_access_key
    attr_accessor :topic_arn

    ALL_FIELD ||= ["access_key_id", "endpoint_url", "region", "secret_access_key", "topic_arn"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SnsConfig` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SnsConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["endpoint_url"] = attributes["endpointUrl"]
      attrs["region"] = attributes["region"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["topic_arn"] = attributes["topicArn"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) if @access_key_id
      out["endpointUrl"] = Svix::serialize_primitive(@endpoint_url) if @endpoint_url
      out["region"] = Svix::serialize_primitive(@region) if @region
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) if @secret_access_key
      out["topicArn"] = Svix::serialize_primitive(@topic_arn) if @topic_arn
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
