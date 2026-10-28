# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  # Configuration parameters for defining a PostHog sink.
  class PostHogConfigIn
    # The PostHog ingestion domain.
    #
    # This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
    # and your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
    # (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
    attr_accessor :domain
    # The PostHog project API key.
    #
    # Sent as the `api_key` field in the body of each `/batch/` request.
    attr_accessor :project_token

    ALL_FIELD ||= ["domain", "project_token"].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::PostHogConfigIn` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::PostHogConfigIn")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["domain"] = attributes["domain"]
      attrs["project_token"] = attributes["projectToken"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["domain"] = Svix::serialize_primitive(@domain) unless @domain.nil?
      out["projectToken"] = Svix::serialize_primitive(@project_token) unless @project_token.nil?
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end
  end
end
