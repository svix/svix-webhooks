# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class SnsConfigOut
    attr_accessor :topic_arn
    attr_accessor :region
    attr_accessor :access_key_id
    attr_accessor :role_arn
    attr_accessor :external_id

    ALL_FIELD ||= ["topic_arn", "region", "access_key_id", "role_arn", "external_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SnsConfigOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SnsConfigOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["topic_arn"] = attributes["topicArn"]
      attrs["region"] = attributes["region"]
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["role_arn"] = attributes["roleArn"]
      attrs["external_id"] = attributes["externalId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["topicArn"] = Svix::serialize_primitive(@topic_arn) unless @topic_arn.nil?
      out["region"] = Svix::serialize_primitive(@region) unless @region.nil?
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) unless @access_key_id.nil?
      out["roleArn"] = Svix::serialize_primitive(@role_arn) unless @role_arn.nil?
      out["externalId"] = Svix::serialize_primitive(@external_id) unless @external_id.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
