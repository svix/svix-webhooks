# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AzureBlobStorageConfig
    attr_accessor :container
    attr_accessor :account
    attr_accessor :access_key

    ALL_FIELD ||= ["container", "account", "access_key"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::AzureBlobStorageConfig` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AzureBlobStorageConfig")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["container"] = attributes["container"]
      attrs["account"] = attributes["account"]
      attrs["access_key"] = attributes["accessKey"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["container"] = Svix::serialize_primitive(@container) if @container
      out["account"] = Svix::serialize_primitive(@account) if @account
      out["accessKey"] = Svix::serialize_primitive(@access_key) if @access_key
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
