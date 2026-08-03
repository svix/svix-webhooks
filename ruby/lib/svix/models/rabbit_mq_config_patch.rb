# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RabbitMqConfigPatch
    attr_accessor :routing_key
    attr_accessor :uri

    ALL_FIELD ||= ["routing_key", "uri"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RabbitMqConfigPatch` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RabbitMqConfigPatch")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["routing_key"] = attributes["routingKey"]
      attrs["uri"] = attributes["uri"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["routingKey"] = Svix::serialize_primitive(@routing_key) unless @routing_key.nil?
      out["uri"] = Svix::serialize_primitive(@uri) unless @uri.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
