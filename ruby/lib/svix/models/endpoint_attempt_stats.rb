# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointAttemptStats
    attr_accessor :success
    attr_accessor :fail
    attr_accessor :canceled

    ALL_FIELD ||= ["success", "fail", "canceled"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointAttemptStats` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointAttemptStats")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["success"] = attributes["success"]
      attrs["fail"] = attributes["fail"]
      attrs["canceled"] = attributes["canceled"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["success"] = Svix::serialize_primitive(@success) unless @success.nil?
      out["fail"] = Svix::serialize_primitive(@fail) unless @fail.nil?
      out["canceled"] = Svix::serialize_primitive(@canceled) unless @canceled.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
