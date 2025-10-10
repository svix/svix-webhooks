# frozen_string_literal: true
# This file is @generated
require "json"

require_relative "./amazon_s3_patch_config"
require_relative "./azure_blob_storage_patch_config"
require_relative "./google_cloud_storage_patch_config"
require_relative "./http_patch_config"
require_relative "./otel_tracing_patch_config"
require_relative "./sink_status_in"

module Svix
  class StreamSinkPatchConfig
    class Poller

      def serialize
        Hash.new
      end

      def self.deserialize(attributes = {})
        new
      end
      # Serializes the object to a json string
      # @return String
      def to_json
        JSON.dump(serialize)
      end
    end

    class AzureBlobStorage < AzureBlobStoragePatchConfig
    end

    class OtelTracing < OtelTracingPatchConfig
    end

    class Http < HttpPatchConfig
    end

    class AmazonS3 < AmazonS3PatchConfig
    end

    class GoogleCloudStorage < GoogleCloudStoragePatchConfig
    end
  end

  class StreamSinkPatch
    attr_accessor :batch_size
    attr_accessor :event_types
    attr_accessor :max_wait_secs
    attr_accessor :metadata
    attr_accessor :status
    # The StreamSink's UID.
    attr_accessor :uid
    attr_accessor :config

    ALL_FIELD ||= ["batch_size", "event_types", "max_wait_secs", "metadata", "status", "uid", "config"].freeze
    private_constant :ALL_FIELD
    TYPE_TO_NAME = {
      StreamSinkPatchConfig::Poller => "poller",
      StreamSinkPatchConfig::AzureBlobStorage => "azureBlobStorage",
      StreamSinkPatchConfig::OtelTracing => "otelTracing",
      StreamSinkPatchConfig::Http => "http",
      StreamSinkPatchConfig::AmazonS3 => "amazonS3",
      StreamSinkPatchConfig::GoogleCloudStorage => "googleCloudStorage"
    }
    private_constant :TYPE_TO_NAME
    NAME_TO_TYPE = TYPE_TO_NAME.invert
    private_constant :NAME_TO_TYPE

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::StreamSinkPatch` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamSinkPatch")
        end

        if k == "config"
          unless TYPE_TO_NAME.key?(v.class)
            fail(ArgumentError, "The field #{k} can't be a `#{v.class}` expected one of #{TYPE_TO_NAME.keys}")
          end

          instance_variable_set("@__enum_discriminator", TYPE_TO_NAME[v.class])
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end

      if @__enum_discriminator.nil?
        fail(ArgumentError, "Required config field was not set")
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["batch_size"] = attributes["batchSize"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["max_wait_secs"] = attributes["maxWaitSecs"]
      attrs["metadata"] = attributes["metadata"]
      attrs["status"] = Svix::SinkStatusIn.deserialize(attributes["status"]) if attributes["status"]
      attrs["uid"] = attributes["uid"]
      unless NAME_TO_TYPE.key?(attributes["type"])
        fail(ArgumentError, "Invalid type `#{attributes["type"]}` expected on of #{NAME_TO_TYPE.keys}")
      end

      unless attributes.key?("config")
        fail(ArgumentError, "Missing required field config")
      end

      attrs["config"] = NAME_TO_TYPE[attributes["type"]].deserialize(attributes["config"])
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["batchSize"] = Svix::serialize_primitive(@batch_size) if @__batch_size_is_defined
      out["eventTypes"] = Svix::serialize_primitive(@event_types) if @event_types
      out["maxWaitSecs"] = Svix::serialize_primitive(@max_wait_secs) if @__max_wait_secs_is_defined
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["status"] = Svix::serialize_schema_ref(@status) if @__status_is_defined
      out["uid"] = Svix::serialize_primitive(@uid) if @__uid_is_defined
      out["type"] = @__enum_discriminator
      out["config"] = @config.serialize
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end

  end
end
