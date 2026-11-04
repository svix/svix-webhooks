# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration parameters for defining a Segment sink.
  class SegmentSinkConfigIn
    # The Segment ingestion host.
    #
    # This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
    # for EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint
    # under this host.
    attr_accessor :host
    # The Segment source write key.
    #
    # Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
    # `/v1/batch` request.
    attr_accessor :write_key

    ALL_FIELD ||= ["host", "write_key"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::SegmentSinkConfigIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::SegmentSinkConfigIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["host"] = attributes["host"]
      attrs["write_key"] = attributes["writeKey"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["host"] = Svix::serialize_primitive(@host) unless @host.nil?
      out["writeKey"] = Svix::serialize_primitive(@write_key) unless @write_key.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
