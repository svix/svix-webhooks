# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AzureBlobStoragePatchConfig
    attr_accessor :access_key
    attr_accessor :account
    attr_accessor :container

    ALL_FIELD ||= ["access_key", "account", "container"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::AzureBlobStoragePatchConfig` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AzureBlobStoragePatchConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["access_key"] = attributes["accessKey"]
      attrs["account"] = attributes["account"]
      attrs["container"] = attributes["container"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["accessKey"] = Svix::serialize_primitive(@access_key) if @access_key
      out["account"] = Svix::serialize_primitive(@account) if @account
      out["container"] = Svix::serialize_primitive(@container) if @container
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
