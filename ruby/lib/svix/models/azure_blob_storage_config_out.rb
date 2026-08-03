# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class AzureBlobStorageConfigOut
    attr_accessor :container
    attr_accessor :account

    ALL_FIELD ||= ["container", "account"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::AzureBlobStorageConfigOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::AzureBlobStorageConfigOut")
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
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["container"] = Svix::serialize_primitive(@container) unless @container.nil?
      out["account"] = Svix::serialize_primitive(@account) unless @account.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
