# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RotateSubscriptionIn2
    attr_accessor :signing_secret

    ALL_FIELD ||= ["signing_secret"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::RotateSubscriptionIn2` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RotateSubscriptionIn2")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      if attributes["signingSecret"]
        attrs["signing_secret"] = Svix::EndpointSecretRotateIn.deserialize(attributes["signingSecret"])
      end

      new(attrs)
    end

    def serialize
      out = Hash.new
      out["signingSecret"] = Svix::serialize_schema_ref(@signing_secret) unless @signing_secret.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
