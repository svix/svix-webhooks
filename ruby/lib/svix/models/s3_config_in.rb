# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class S3ConfigIn
    attr_accessor :bucket
    # Access key ID.
    #
    # Required (along with `secret_access_key`) if `role_arn` is null
    attr_accessor :access_key_id
    # Secret access key.
    #
    # Required (along with `access_key_id`) if `role_arn` is null
    attr_accessor :secret_access_key
    # The region of the S3 bucket
    #
    # Currently a required field, but marked as optional because we may infer it from other fields in the future.
    attr_accessor :region
    attr_accessor :endpoint_url
    # Role ARN for delegated authentication
    attr_accessor :role_arn
    # Shared secret passed as the STS ExternalId.
    #
    # Recommended if `role_arn` is not null.
    attr_accessor :external_id

    ALL_FIELD ||= ["bucket", "access_key_id", "secret_access_key", "region", "endpoint_url", "role_arn", "external_id"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::S3ConfigIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::S3ConfigIn")
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
      attrs["region"] = attributes["region"]
      attrs["endpoint_url"] = attributes["endpointUrl"]
      attrs["role_arn"] = attributes["roleArn"]
      attrs["external_id"] = attributes["externalId"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["bucket"] = Svix::serialize_primitive(@bucket) unless @bucket.nil?
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) unless @access_key_id.nil?
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) unless @secret_access_key.nil?
      out["region"] = Svix::serialize_primitive(@region) unless @region.nil?
      out["endpointUrl"] = Svix::serialize_primitive(@endpoint_url) unless @endpoint_url.nil?
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
