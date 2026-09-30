# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RabbitMqConfigOut
    attr_accessor :uri
    attr_accessor :routing_key
    attr_accessor :mandatory

    ALL_FIELD ||= ["uri", "routing_key", "mandatory"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RabbitMqConfigOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RabbitMqConfigOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["uri"] = attributes["uri"]
      attrs["routing_key"] = attributes["routingKey"]
      attrs["mandatory"] = attributes["mandatory"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["uri"] = Svix::serialize_primitive(@uri) unless @uri.nil?
      out["routingKey"] = Svix::serialize_primitive(@routing_key) unless @routing_key.nil?
      out["mandatory"] = Svix::serialize_primitive(@mandatory) unless @mandatory.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
