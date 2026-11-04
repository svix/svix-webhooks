# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class BatchCreateMessagesIn
    attr_accessor :messages

    ALL_FIELD ||= ["messages"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::BatchCreateMessagesIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::BatchCreateMessagesIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["messages"] = attributes["messages"].map { |v| Svix::BatchMessageIn.deserialize(v) }
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["messages"] = @messages.map { |v| v.serialize } unless @messages.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
