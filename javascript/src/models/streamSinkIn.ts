// this file is @generated
import {
  type AzureBlobStorageConfigIn,
  AzureBlobStorageConfigInSerializer,
} from "./azureBlobStorageConfigIn";
import { type BigQueryConfigIn, BigQueryConfigInSerializer } from "./bigQueryConfigIn";
import {
  type ClickhouseConfigIn,
  ClickhouseConfigInSerializer,
} from "./clickhouseConfigIn";
import {
  type EventBridgeConfigIn,
  EventBridgeConfigInSerializer,
} from "./eventBridgeConfigIn";
import {
  type GoogleCloudPubSubConfigIn,
  GoogleCloudPubSubConfigInSerializer,
} from "./googleCloudPubSubConfigIn";
import {
  type GoogleCloudStorageConfigIn,
  GoogleCloudStorageConfigInSerializer,
} from "./googleCloudStorageConfigIn";
import {
  type OtelTracingConfigIn,
  OtelTracingConfigInSerializer,
} from "./otelTracingConfigIn";
import { type RabbitMqConfigIn, RabbitMqConfigInSerializer } from "./rabbitMqConfigIn";
import { type RedshiftConfigIn, RedshiftConfigInSerializer } from "./redshiftConfigIn";
import { type S3ConfigIn, S3ConfigInSerializer } from "./s3ConfigIn";
import { type SinkHttpConfigIn, SinkHttpConfigInSerializer } from "./sinkHttpConfigIn";
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
import { type SnowflakeConfigIn, SnowflakeConfigInSerializer } from "./snowflakeConfigIn";
import { type SnsConfigIn, SnsConfigInSerializer } from "./snsConfigIn";
import { type SqsConfigIn, SqsConfigInSerializer } from "./sqsConfigIn";

interface _StreamSinkInFields {
  /** An optional unique identifier for the sink. */
  uid?: string | null;
  /**
   * Whether the sink will receive events.
   *
   * If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.
   *
   * If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled.
   */
  status?: SinkStatusIn;
  /** How many events will be batched in a request to the Sink. */
  batchSize?: number;
  /**
   * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
   *
   * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.
   *
   * Note that we will never send an empty batch of events to the Sink.
   */
  maxWaitSecs?: number;
  /** A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events. */
  eventTypes?: string[];
  channels?: string[];
  metadata?: { [key: string]: string };
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface StreamSinkInPollerConfig {}

interface StreamSinkInPoller {
  type: "poller";
  config?: StreamSinkInPollerConfig;
}

interface StreamSinkInAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfigIn;
}

interface StreamSinkInOtelTracing {
  type: "otelTracing";
  config: OtelTracingConfigIn;
}

interface StreamSinkInHttp {
  type: "http";
  config: SinkHttpConfigIn;
}

interface StreamSinkInAmazonS3 {
  type: "amazonS3";
  config: S3ConfigIn;
}

interface StreamSinkInGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfigIn;
}

interface StreamSinkInGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfigIn;
}

interface StreamSinkInSqs {
  type: "sqs";
  config: SqsConfigIn;
}

interface StreamSinkInSns {
  type: "sns";
  config: SnsConfigIn;
}

interface StreamSinkInBigQuery {
  type: "bigQuery";
  config: BigQueryConfigIn;
}

interface StreamSinkInClickhouse {
  type: "clickhouse";
  config: ClickhouseConfigIn;
}

interface StreamSinkInEventBridge {
  type: "eventBridge";
  config: EventBridgeConfigIn;
}

interface StreamSinkInSnowflake {
  type: "snowflake";
  config: SnowflakeConfigIn;
}

interface StreamSinkInRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfigIn;
}

interface StreamSinkInRedshift {
  type: "redshift";
  config: RedshiftConfigIn;
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
          return AzureBlobStorageConfigInSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingConfigInSerializer._fromJsonObject(object["config"]);
        case "http":
          return SinkHttpConfigInSerializer._fromJsonObject(object["config"]);
        case "amazonS3":
          return S3ConfigInSerializer._fromJsonObject(object["config"]);
        case "googleCloudStorage":
          return GoogleCloudStorageConfigInSerializer._fromJsonObject(object["config"]);
        case "googleCloudPubSub":
          return GoogleCloudPubSubConfigInSerializer._fromJsonObject(object["config"]);
        case "sqs":
          return SqsConfigInSerializer._fromJsonObject(object["config"]);
        case "sns":
          return SnsConfigInSerializer._fromJsonObject(object["config"]);
        case "bigQuery":
          return BigQueryConfigInSerializer._fromJsonObject(object["config"]);
        case "clickhouse":
          return ClickhouseConfigInSerializer._fromJsonObject(object["config"]);
        case "eventBridge":
          return EventBridgeConfigInSerializer._fromJsonObject(object["config"]);
        case "snowflake":
          return SnowflakeConfigInSerializer._fromJsonObject(object["config"]);
        case "rabbitMq":
          return RabbitMqConfigInSerializer._fromJsonObject(object["config"]);
        case "redshift":
          return RedshiftConfigInSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }

    return {
      type,
      config: getConfig(type),
      uid: object["uid"],
      status:
        object["status"] != null
          ? SinkStatusInSerializer._fromJsonObject(object["status"])
          : undefined,
      batchSize: object["batchSize"],
      maxWaitSecs: object["maxWaitSecs"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      metadata: object["metadata"],
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
        config = AzureBlobStorageConfigInSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingConfigInSerializer._toJsonObject(self.config);
        break;
      case "http":
        config = SinkHttpConfigInSerializer._toJsonObject(self.config);
        break;
      case "amazonS3":
        config = S3ConfigInSerializer._toJsonObject(self.config);
        break;
      case "googleCloudStorage":
        config = GoogleCloudStorageConfigInSerializer._toJsonObject(self.config);
        break;
      case "googleCloudPubSub":
        config = GoogleCloudPubSubConfigInSerializer._toJsonObject(self.config);
        break;
      case "sqs":
        config = SqsConfigInSerializer._toJsonObject(self.config);
        break;
      case "sns":
        config = SnsConfigInSerializer._toJsonObject(self.config);
        break;
      case "bigQuery":
        config = BigQueryConfigInSerializer._toJsonObject(self.config);
        break;
      case "clickhouse":
        config = ClickhouseConfigInSerializer._toJsonObject(self.config);
        break;
      case "eventBridge":
        config = EventBridgeConfigInSerializer._toJsonObject(self.config);
        break;
      case "snowflake":
        config = SnowflakeConfigInSerializer._toJsonObject(self.config);
        break;
      case "rabbitMq":
        config = RabbitMqConfigInSerializer._toJsonObject(self.config);
        break;
      case "redshift":
        config = RedshiftConfigInSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      uid: self.uid,
      status:
        self.status != null
          ? SinkStatusInSerializer._toJsonObject(self.status)
          : undefined,
      batchSize: self.batchSize,
      maxWaitSecs: self.maxWaitSecs,
      eventTypes: self.eventTypes,
      channels: self.channels,
      metadata: self.metadata,
    };
  },
};
