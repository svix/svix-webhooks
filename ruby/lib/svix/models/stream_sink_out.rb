# frozen_string_literal: true
# This file is @generated
require "json"

require_relative "./azure_blob_storage_config"
require_relative "./google_cloud_storage_config"
require_relative "./s3_config"
require_relative "./sink_http_config"
require_relative "./sink_otel_v1_config"
require_relative "./sink_status"

module Svix
  class StreamSinkOutConfig
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

    class AzureBlobStorage < AzureBlobStorageConfig
    end

    class OtelTracing < SinkOtelV1Config
    end

    class Http < SinkHttpConfig
    end

    class AmazonS3 < S3Config
    end

    class GoogleCloudStorage < GoogleCloudStorageConfig
    end
  end

  class StreamSinkOut
    attr_accessor :batch_size
    attr_accessor :created_at
    attr_accessor :current_iterator
    attr_accessor :event_types
    attr_accessor :failure_reason
    # The sink's ID.
    attr_accessor :id
    attr_accessor :max_wait_secs
    attr_accessor :metadata
    attr_accessor :next_retry_at
    attr_accessor :status
    # The sink's UID.
    attr_accessor :uid
    attr_accessor :updated_at
    attr_accessor :config

    ALL_FIELD ||= [
      "batch_size",
      "created_at",
      "current_iterator",
      "event_types",
      "failure_reason",
      "id",
      "max_wait_secs",
      "metadata",
      "next_retry_at",
      "status",
      "uid",
      "updated_at",
      "config"
    ].freeze
    private_constant :ALL_FIELD
    TYPE_TO_NAME = {
      StreamSinkOutConfig::Poller => "poller",
      StreamSinkOutConfig::AzureBlobStorage => "azureBlobStorage",
      StreamSinkOutConfig::OtelTracing => "otelTracing",
      StreamSinkOutConfig::Http => "http",
      StreamSinkOutConfig::AmazonS3 => "amazonS3",
      StreamSinkOutConfig::GoogleCloudStorage => "googleCloudStorage"
    }
    private_constant :TYPE_TO_NAME
    NAME_TO_TYPE = TYPE_TO_NAME.invert
    private_constant :NAME_TO_TYPE

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::StreamSinkOut` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::StreamSinkOut")
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
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["current_iterator"] = attributes["currentIterator"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["failure_reason"] = attributes["failureReason"]
      attrs["id"] = attributes["id"]
      attrs["max_wait_secs"] = attributes["maxWaitSecs"]
      attrs["metadata"] = attributes["metadata"]
      attrs["next_retry_at"] = DateTime.rfc3339(attributes["nextRetryAt"]).to_time if attributes["nextRetryAt"]
      attrs["status"] = Svix::SinkStatus.deserialize(attributes["status"])
      attrs["uid"] = attributes["uid"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
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
      out["batchSize"] = Svix::serialize_primitive(@batch_size) if @batch_size
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["currentIterator"] = Svix::serialize_primitive(@current_iterator) if @current_iterator
      out["eventTypes"] = Svix::serialize_primitive(@event_types) if @event_types
      out["failureReason"] = Svix::serialize_primitive(@failure_reason) if @failure_reason
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["maxWaitSecs"] = Svix::serialize_primitive(@max_wait_secs) if @max_wait_secs
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["nextRetryAt"] = Svix::serialize_primitive(@next_retry_at) if @next_retry_at
      out["status"] = Svix::serialize_schema_ref(@status) if @status
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
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
