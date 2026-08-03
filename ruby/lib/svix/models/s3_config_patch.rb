# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class S3ConfigPatch
    attr_accessor :bucket
    attr_accessor :access_key_id
    attr_accessor :secret_access_key
    attr_accessor :role_arn
    attr_accessor :external_id
    attr_accessor :region
    attr_accessor :endpoint_url

    ALL_FIELD ||= ["bucket", "access_key_id", "secret_access_key", "role_arn", "external_id", "region", "endpoint_url"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::S3ConfigPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::S3ConfigPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["bucket"] = attributes["bucket"]
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["role_arn"] = attributes["roleArn"]
      attrs["external_id"] = attributes["externalId"]
      attrs["region"] = attributes["region"]
      attrs["endpoint_url"] = attributes["endpointUrl"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["bucket"] = Svix::serialize_primitive(@bucket) unless @bucket.nil?
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) unless @access_key_id.nil?
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) unless @secret_access_key.nil?
      out["roleArn"] = Svix::serialize_primitive(@role_arn) unless @role_arn.nil?
      out["externalId"] = Svix::serialize_primitive(@external_id) unless @external_id.nil?
      out["region"] = Svix::serialize_primitive(@region) unless @region.nil?
      out["endpointUrl"] = Svix::serialize_primitive(@endpoint_url) unless @endpoint_url.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
