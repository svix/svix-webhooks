# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class ElasticSearchConfigPatch
    attr_accessor :index_name
    attr_accessor :url
    attr_accessor :api_key
    attr_accessor :refresh

    ALL_FIELD ||= ["index_name", "url", "api_key", "refresh"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::ElasticSearchConfigPatch` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::ElasticSearchConfigPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["index_name"] = attributes["indexName"]
      attrs["url"] = attributes["url"]
      attrs["api_key"] = attributes["apiKey"]
      attrs["refresh"] = attributes["refresh"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["indexName"] = Svix::serialize_primitive(@index_name) unless @index_name.nil?
      out["url"] = Svix::serialize_primitive(@url) unless @url.nil?
      out["apiKey"] = Svix::serialize_primitive(@api_key) unless @api_key.nil?
      out["refresh"] = Svix::serialize_primitive(@refresh) unless @refresh.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
