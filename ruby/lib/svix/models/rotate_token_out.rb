# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class RotateTokenOut
    attr_accessor :ingest_url

    ALL_FIELD ||= ["ingest_url"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::RotateTokenOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::RotateTokenOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["ingest_url"] = attributes["ingestUrl"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["ingestUrl"] = Svix::serialize_primitive(@ingest_url) if @ingest_url
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
