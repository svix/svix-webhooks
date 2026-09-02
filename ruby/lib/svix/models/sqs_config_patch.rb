# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class SqsConfigPatch
    attr_accessor :queue_url
    attr_accessor :region
    attr_accessor :access_key_id
    attr_accessor :secret_access_key
    attr_accessor :endpoint_url

    ALL_FIELD ||= ["queue_url", "region", "access_key_id", "secret_access_key", "endpoint_url"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SqsConfigPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SqsConfigPatch")
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
      attrs["endpoint_url"] = attributes["endpointUrl"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["queueUrl"] = Svix::serialize_primitive(@queue_url) unless @queue_url.nil?
      out["region"] = Svix::serialize_primitive(@region) unless @region.nil?
      out["accessKeyId"] = Svix::serialize_primitive(@access_key_id) unless @access_key_id.nil?
      out["secretAccessKey"] = Svix::serialize_primitive(@secret_access_key) unless @secret_access_key.nil?
      out["endpointUrl"] = Svix::serialize_primitive(@endpoint_url) if @__endpoint_url_is_defined
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
