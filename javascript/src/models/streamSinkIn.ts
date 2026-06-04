// this file is @generated
import {
  type AzureBlobStorageConfig,
  AzureBlobStorageConfigSerializer,
} from "./azureBlobStorageConfig";
import { type BigQueryConfig, BigQueryConfigSerializer } from "./bigQueryConfig";
import { type ClickhouseConfig, ClickhouseConfigSerializer } from "./clickhouseConfig";
import { type EventBridgeConfig, EventBridgeConfigSerializer } from "./eventBridgeConfig";
import {
  type GoogleCloudPubSubConfig,
  GoogleCloudPubSubConfigSerializer,
} from "./googleCloudPubSubConfig";
import {
  type GoogleCloudStorageConfig,
  GoogleCloudStorageConfigSerializer,
} from "./googleCloudStorageConfig";
import { type RabbitMqConfig, RabbitMqConfigSerializer } from "./rabbitMqConfig";
import { type RedshiftConfig, RedshiftConfigSerializer } from "./redshiftConfig";
import { type S3Config, S3ConfigSerializer } from "./s3Config";
import { type SinkHttpConfig, SinkHttpConfigSerializer } from "./sinkHttpConfig";
import { type SinkOtelV1Config, SinkOtelV1ConfigSerializer } from "./sinkOtelV1Config";
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
import { type SnowflakeConfig, SnowflakeConfigSerializer } from "./snowflakeConfig";
import { type SnsConfig, SnsConfigSerializer } from "./snsConfig";
import { type SqsConfig, SqsConfigSerializer } from "./sqsConfig";
interface _StreamSinkInFields {
  /** How many events will be batched in a request to the Sink. */
  batchSize?: number;
  /** A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events. */
  eventTypes?: string[];
  /**
   * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
   *
   * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.
   *
   * Note that we will never send an empty batch of events to the Sink.
   */
  maxWaitSecs?: number;
  metadata?: { [key: string]: string };
  /**
   * Whether the sink will receive events.
   *
   * If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.
   *
   * If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled.
   */
  status?: SinkStatusIn;
  /** An optional unique identifier for the sink. */
  uid?: string | null;
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface StreamSinkInPollerConfig {}

interface StreamSinkInPoller {
  type: "poller";
  config?: StreamSinkInPollerConfig;
}

interface StreamSinkInAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfig;
}

interface StreamSinkInOtelTracing {
  type: "otelTracing";
  config: SinkOtelV1Config;
}

interface StreamSinkInHttp {
  type: "http";
  config: SinkHttpConfig;
}

interface StreamSinkInAmazonS3 {
  type: "amazonS3";
  config: S3Config;
}

interface StreamSinkInGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfig;
}

interface StreamSinkInGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfig;
}

interface StreamSinkInSqs {
  type: "sqs";
  config: SqsConfig;
}

interface StreamSinkInSns {
  type: "sns";
  config: SnsConfig;
}

interface StreamSinkInBigQuery {
  type: "bigQuery";
  config: BigQueryConfig;
}

interface StreamSinkInClickhouse {
  type: "clickhouse";
  config: ClickhouseConfig;
}

interface StreamSinkInEventBridge {
  type: "eventBridge";
  config: EventBridgeConfig;
}

interface StreamSinkInSnowflake {
  type: "snowflake";
  config: SnowflakeConfig;
}

interface StreamSinkInRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfig;
}

interface StreamSinkInRedshift {
  type: "redshift";
  config: RedshiftConfig;
}

export type StreamSinkIn = _StreamSinkInFields &
  (
    | StreamSinkInPoller
    | StreamSinkInAzureBlobStorage
    | StreamSinkInOtelTracing
    | StreamSinkInHttp
    | StreamSinkInAmazonS3
    | StreamSinkInGoogleCloudStorage
    | StreamSinkInGoogleCloudPubSub
    | StreamSinkInSqs
    | StreamSinkInSns
    | StreamSinkInBigQuery
    | StreamSinkInClickhouse
    | StreamSinkInEventBridge
    | StreamSinkInSnowflake
    | StreamSinkInRabbitMq
    | StreamSinkInRedshift
  );

export const StreamSinkInSerializer = {
  _fromJsonObject(object: any): StreamSinkIn {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "poller":
          return {};
        case "azureBlobStorage":
          return AzureBlobStorageConfigSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return SinkOtelV1ConfigSerializer._fromJsonObject(object["config"]);
        case "http":
          return SinkHttpConfigSerializer._fromJsonObject(object["config"]);
        case "amazonS3":
          return S3ConfigSerializer._fromJsonObject(object["config"]);
        case "googleCloudStorage":
          return GoogleCloudStorageConfigSerializer._fromJsonObject(object["config"]);
        case "googleCloudPubSub":
          return GoogleCloudPubSubConfigSerializer._fromJsonObject(object["config"]);
        case "sqs":
          return SqsConfigSerializer._fromJsonObject(object["config"]);
        case "sns":
          return SnsConfigSerializer._fromJsonObject(object["config"]);
        case "bigQuery":
          return BigQueryConfigSerializer._fromJsonObject(object["config"]);
        case "clickhouse":
          return ClickhouseConfigSerializer._fromJsonObject(object["config"]);
        case "eventBridge":
          return EventBridgeConfigSerializer._fromJsonObject(object["config"]);
        case "snowflake":
          return SnowflakeConfigSerializer._fromJsonObject(object["config"]);
        case "rabbitMq":
          return RabbitMqConfigSerializer._fromJsonObject(object["config"]);
        case "redshift":
          return RedshiftConfigSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }

    return {
      type,
      config: getConfig(type),
      batchSize: object["batchSize"],
      eventTypes: object["eventTypes"],
      maxWaitSecs: object["maxWaitSecs"],
      metadata: object["metadata"],
      status:
        object["status"] != null
          ? SinkStatusInSerializer._fromJsonObject(object["status"])
          : undefined,
      uid: object["uid"],
    };
  },

  _toJsonObject(self: StreamSinkIn): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "poller":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStorageConfigSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = SinkOtelV1ConfigSerializer._toJsonObject(self.config);
        break;
      case "http":
        config = SinkHttpConfigSerializer._toJsonObject(self.config);
        break;
      case "amazonS3":
        config = S3ConfigSerializer._toJsonObject(self.config);
        break;
      case "googleCloudStorage":
        config = GoogleCloudStorageConfigSerializer._toJsonObject(self.config);
        break;
      case "googleCloudPubSub":
        config = GoogleCloudPubSubConfigSerializer._toJsonObject(self.config);
        break;
      case "sqs":
        config = SqsConfigSerializer._toJsonObject(self.config);
        break;
      case "sns":
        config = SnsConfigSerializer._toJsonObject(self.config);
        break;
      case "bigQuery":
        config = BigQueryConfigSerializer._toJsonObject(self.config);
        break;
      case "clickhouse":
        config = ClickhouseConfigSerializer._toJsonObject(self.config);
        break;
      case "eventBridge":
        config = EventBridgeConfigSerializer._toJsonObject(self.config);
        break;
      case "snowflake":
        config = SnowflakeConfigSerializer._toJsonObject(self.config);
        break;
      case "rabbitMq":
        config = RabbitMqConfigSerializer._toJsonObject(self.config);
        break;
      case "redshift":
        config = RedshiftConfigSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      batchSize: self.batchSize,
      eventTypes: self.eventTypes,
      maxWaitSecs: self.maxWaitSecs,
      metadata: self.metadata,
      status:
        self.status != null
          ? SinkStatusInSerializer._toJsonObject(self.status)
          : undefined,
      uid: self.uid,
    };
  },
};
