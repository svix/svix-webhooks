# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration for an SQS sink.
  class SqsConfigIn
    attr_accessor :queue_url
    # The region of the SQS queue.
    #
    # Currently a required field, but marked as optional because we may infer it from other fields in the future.
    attr_accessor :region
    # Access key ID.
    #
    # Required (along with `secret_access_key`) if `role_arn` is blank.
    attr_accessor :access_key_id
    # Secret access key.
    #
    # Required (along with `access_key_id`) if `role_arn` is blank.
    attr_accessor :secret_access_key
    # Role ARN for delegated authentication
    attr_accessor :role_arn
    # Shared secret passed as the STS ExternalId.
    #
    # Can only be set if `role_arn` is Some
    attr_accessor :external_id
    attr_accessor :endpoint_url

    ALL_FIELD ||= [
      "queue_url",
      "region",
      "access_key_id",
      "secret_access_key",
      "role_arn",
      "external_id",
      "endpoint_url"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SqsConfigIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SqsConfigIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["queue_url"] = attributes["queueUrl"]
      attrs["region"] = attributes["region"]
      attrs["access_key_id"] = attributes["accessKeyId"]
      attrs["secret_access_key"] = attributes["secretAccessKey"]
      attrs["role_arn"] = attributes["roleArn"]
      attrs["external_id"] = attributes["externalId"]
      attrs["endpoint_url"] = attributes["endpointUrl"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["queueUrl"] = Svix::serialize_primitive(@queue_url) unless @queue_url.nil?
      out["region"] = Svix::serialize_primitive(@region) unless @region.nil?
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) unless @access_key_id.nil?
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) unless @secret_access_key.nil?
      out["roleArn"] = Svix::serialize_primitive(@role_arn) unless @role_arn.nil?
      out["externalId"] = Svix::serialize_primitive(@external_id) unless @external_id.nil?
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
