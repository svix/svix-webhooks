# frozen_string_literal: true
# This file is @generated
require "json"

require_relative "./azure_blob_storage_config_out"
require_relative "./big_query_config_out"
require_relative "./clickhouse_config_out"
require_relative "./event_bridge_config_out"
require_relative "./google_cloud_pub_sub_config_out"
require_relative "./google_cloud_storage_config_out"
require_relative "./otel_tracing_config_out"
require_relative "./postgres_config_out"
require_relative "./rabbit_mq_config_out"
require_relative "./redshift_config_out"
require_relative "./s3_config_out"
require_relative "./sink_http_config_out"
require_relative "./sink_status"
require_relative "./snowflake_config_out"
require_relative "./sns_config_out"
require_relative "./sqs_config_out"

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
      def to_json(*args)
        serialize.to_json(*args)
      end
    end

    class AzureBlobStorage < AzureBlobStorageConfigOut
    end

    class OtelTracing < OtelTracingConfigOut
    end

    class Http < SinkHttpConfigOut
    end

    class AmazonS3 < S3ConfigOut
    end

    class Snowflake < SnowflakeConfigOut
    end

    class GoogleCloudStorage < GoogleCloudStorageConfigOut
    end

    class GoogleCloudPubSub < GoogleCloudPubSubConfigOut
    end

    class Redshift < RedshiftConfigOut
    end

    class BigQuery < BigQueryConfigOut
    end

    class Clickhouse < ClickhouseConfigOut
    end

    class RabbitMq < RabbitMqConfigOut
    end

    class Sqs < SqsConfigOut
    end

    class EventBridge < EventBridgeConfigOut
    end

    class Sns < SnsConfigOut
    end

    class Postgres < PostgresConfigOut
    end
  end

  class StreamSinkOut
    # The sink's ID.
    attr_accessor :id
    # The sink's UID.
    attr_accessor :uid
    attr_accessor :status
    attr_accessor :current_iterator
    attr_accessor :failure_reason
    attr_accessor :created_at
    attr_accessor :updated_at
    attr_accessor :batch_size
    attr_accessor :max_wait_secs
    attr_accessor :event_types
    attr_accessor :channels
    attr_accessor :next_retry_at
    attr_accessor :metadata
    attr_accessor :config

    ALL_FIELD ||= [
      "id",
      "uid",
      "status",
      "current_iterator",
      "failure_reason",
      "created_at",
      "updated_at",
      "batch_size",
      "max_wait_secs",
      "event_types",
      "channels",
      "next_retry_at",
      "metadata",
      "config"
    ].freeze
    private_constant :ALL_FIELD
    TYPE_TO_NAME = {
      StreamSinkOutConfig::Poller => "poller",
      StreamSinkOutConfig::AzureBlobStorage => "azureBlobStorage",
      StreamSinkOutConfig::OtelTracing => "otelTracing",
      StreamSinkOutConfig::Http => "http",
      StreamSinkOutConfig::AmazonS3 => "amazonS3",
      StreamSinkOutConfig::Snowflake => "snowflake",
      StreamSinkOutConfig::GoogleCloudStorage => "googleCloudStorage",
      StreamSinkOutConfig::GoogleCloudPubSub => "googleCloudPubSub",
      StreamSinkOutConfig::Redshift => "redshift",
      StreamSinkOutConfig::BigQuery => "bigQuery",
      StreamSinkOutConfig::Clickhouse => "clickhouse",
      StreamSinkOutConfig::RabbitMq => "rabbitMq",
      StreamSinkOutConfig::Sqs => "sqs",
      StreamSinkOutConfig::EventBridge => "eventBridge",
      StreamSinkOutConfig::Sns => "sns",
      StreamSinkOutConfig::Postgres => "postgres"
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

        if k.to_s == "config"
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
      attrs["id"] = attributes["id"]
      attrs["uid"] = attributes["uid"]
      attrs["status"] = Svix::SinkStatus.deserialize(attributes["status"])
      attrs["current_iterator"] = attributes["currentIterator"]
      attrs["failure_reason"] = attributes["failureReason"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["batch_size"] = attributes["batchSize"]
      attrs["max_wait_secs"] = attributes["maxWaitSecs"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["channels"] = attributes["channels"]
      attrs["next_retry_at"] = DateTime.rfc3339(attributes["nextRetryAt"]).to_time if attributes["nextRetryAt"]
      attrs["metadata"] = attributes["metadata"]
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
      out["id"] = Svix::serialize_primitive(@id) unless @id.nil?
      out["uid"] = Svix::serialize_primitive(@uid) unless @uid.nil?
      out["status"] = Svix::serialize_schema_ref(@status) unless @status.nil?
      out["currentIterator"] = Svix::serialize_primitive(@current_iterator) unless @current_iterator.nil?
      out["failureReason"] = Svix::serialize_primitive(@failure_reason) unless @failure_reason.nil?
      out["createdAt"] = Svix::serialize_primitive(@created_at) unless @created_at.nil?
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) unless @updated_at.nil?
      out["batchSize"] = Svix::serialize_primitive(@batch_size) unless @batch_size.nil?
      out["maxWaitSecs"] = Svix::serialize_primitive(@max_wait_secs) unless @max_wait_secs.nil?
      out["eventTypes"] = Svix::serialize_primitive(@event_types) unless @event_types.nil?
      out["channels"] = Svix::serialize_primitive(@channels) unless @channels.nil?
      out["nextRetryAt"] = Svix::serialize_primitive(@next_retry_at) unless @next_retry_at.nil?
      out["metadata"] = Svix::serialize_primitive(@metadata) unless @metadata.nil?
      out["type"] = @__enum_discriminator
      out["config"] = @config.serialize
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json(*args)
      serialize.to_json(*args)
    end

  end
end
