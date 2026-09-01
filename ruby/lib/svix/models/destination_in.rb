# frozen_string_literal: true
# This file is @generated
require "json"

require_relative "./azure_blob_storage_config_in"
require_relative "./big_query_config_in"
require_relative "./clickhouse_config_in"
require_relative "./event_bridge_config_in"
require_relative "./fifo_endpoint_config_in"
require_relative "./google_cloud_pub_sub_config_in"
require_relative "./google_cloud_storage_config_in"
require_relative "./otel_tracing_config_in"
require_relative "./postgres_config_in"
require_relative "./rabbit_mq_config_in"
require_relative "./redshift_config_in"
require_relative "./s3_config_in"
require_relative "./sink_status_in"
require_relative "./snowflake_config_in"
require_relative "./sns_config_in"
require_relative "./sqs_config_in"

module Svix
  class DestinationInConfig
    class PollingEndpoint

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

    class AzureBlobStorage < AzureBlobStorageConfigIn
    end

    class OtelTracing < OtelTracingConfigIn
    end

    class FifoEndpoint < FifoEndpointConfigIn
    end

    class AmazonS3 < S3ConfigIn
    end

    class GoogleCloudStorage < GoogleCloudStorageConfigIn
    end

    class GoogleCloudPubSub < GoogleCloudPubSubConfigIn
    end

    class Sqs < SqsConfigIn
    end

    class Sns < SnsConfigIn
    end

    class BigQuery < BigQueryConfigIn
    end

    class Clickhouse < ClickhouseConfigIn
    end

    class EventBridge < EventBridgeConfigIn
    end

    class Snowflake < SnowflakeConfigIn
    end

    class RabbitMq < RabbitMqConfigIn
    end

    class Redshift < RedshiftConfigIn
    end

    class Postgres < PostgresConfigIn
    end
  end

  # The destination's type and type-specific configuration.
  class DestinationIn
    # An optional unique identifier for the destination.
    attr_accessor :uid
    # Whether the destination will receive events.
    #
    # If the destination is `enabled`, events sent to the application will be dispatched to the destination in order.
    #
    # If the destination is `disabled`, events will not be dispatched until the destination is reenabled.
    attr_accessor :status
    # How many events will be batched in a request to the destination.
    attr_accessor :batch_size
    # How long to wait before a batch of events is sent, if the `batchSize` is not reached.
    #
    # For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after 10 seconds or 100 events, whichever comes first.
    #
    # Note that an empty batch is never sent to the destination.
    attr_accessor :max_wait_secs
    # A list of event types that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
    attr_accessor :event_types
    # A list of channels that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events.
    attr_accessor :channels
    attr_accessor :metadata
    attr_accessor :config

    ALL_FIELD ||= ["uid", "status", "batch_size", "max_wait_secs", "event_types", "channels", "metadata", "config"].freeze
    private_constant :ALL_FIELD
    TYPE_TO_NAME = {
      DestinationInConfig::PollingEndpoint => "pollingEndpoint",
      DestinationInConfig::AzureBlobStorage => "azureBlobStorage",
      DestinationInConfig::OtelTracing => "otelTracing",
      DestinationInConfig::FifoEndpoint => "fifoEndpoint",
      DestinationInConfig::AmazonS3 => "amazonS3",
      DestinationInConfig::GoogleCloudStorage => "googleCloudStorage",
      DestinationInConfig::GoogleCloudPubSub => "googleCloudPubSub",
      DestinationInConfig::Sqs => "sqs",
      DestinationInConfig::Sns => "sns",
      DestinationInConfig::BigQuery => "bigQuery",
      DestinationInConfig::Clickhouse => "clickhouse",
      DestinationInConfig::EventBridge => "eventBridge",
      DestinationInConfig::Snowflake => "snowflake",
      DestinationInConfig::RabbitMq => "rabbitMq",
      DestinationInConfig::Redshift => "redshift",
      DestinationInConfig::Postgres => "postgres"
    }
    private_constant :TYPE_TO_NAME
    NAME_TO_TYPE = TYPE_TO_NAME.invert
    private_constant :NAME_TO_TYPE

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(
          ArgumentError,
          "The input argument (attributes) must be a hash in `Svix::DestinationIn` new method"
        )
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::DestinationIn")
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
      attrs["uid"] = attributes["uid"]
      attrs["status"] = Svix::SinkStatusIn.deserialize(attributes["status"]) if attributes["status"]
      attrs["batch_size"] = attributes["batchSize"]
      attrs["max_wait_secs"] = attributes["maxWaitSecs"]
      attrs["event_types"] = attributes["eventTypes"]
      attrs["channels"] = attributes["channels"]
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
      out["uid"] = Svix::serialize_primitive(@uid) unless @uid.nil?
      out["status"] = Svix::serialize_schema_ref(@status) unless @status.nil?
      out["batchSize"] = Svix::serialize_primitive(@batch_size) unless @batch_size.nil?
      out["maxWaitSecs"] = Svix::serialize_primitive(@max_wait_secs) unless @max_wait_secs.nil?
      out["eventTypes"] = Svix::serialize_primitive(@event_types) unless @event_types.nil?
      out["channels"] = Svix::serialize_primitive(@channels) unless @channels.nil?
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
