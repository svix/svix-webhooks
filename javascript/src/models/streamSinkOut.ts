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
import { type SinkStatus, SinkStatusSerializer } from "./sinkStatus";
import { type SnowflakeConfig, SnowflakeConfigSerializer } from "./snowflakeConfig";
import { type SnsConfig, SnsConfigSerializer } from "./snsConfig";
import { type SqsConfig, SqsConfigSerializer } from "./sqsConfig";
interface _StreamSinkOutFields {
  batchSize: number;
  createdAt: Date;
  currentIterator: string;
  eventTypes?: string[];
  failureReason?: string | null;
  /** The sink's ID. */
  id: string;
  maxWaitSecs: number;
  metadata: { [key: string]: string };
  nextRetryAt?: Date | null;
  status: SinkStatus;
  /** The sink's UID. */
  uid?: string | null;
  updatedAt: Date;
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface StreamSinkOutPollerConfig {}

interface StreamSinkOutPoller {
  type: "poller";
  config?: StreamSinkOutPollerConfig;
}

interface StreamSinkOutAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfig;
}

interface StreamSinkOutOtelTracing {
  type: "otelTracing";
  config: SinkOtelV1Config;
}

interface StreamSinkOutHttp {
  type: "http";
  config: SinkHttpConfig;
}

interface StreamSinkOutAmazonS3 {
  type: "amazonS3";
  config: S3Config;
}

interface StreamSinkOutGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfig;
}

interface StreamSinkOutGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfig;
}

interface StreamSinkOutSqs {
  type: "sqs";
  config: SqsConfig;
}

interface StreamSinkOutSns {
  type: "sns";
  config: SnsConfig;
}

interface StreamSinkOutBigQuery {
  type: "bigQuery";
  config: BigQueryConfig;
}

interface StreamSinkOutClickhouse {
  type: "clickhouse";
  config: ClickhouseConfig;
}

interface StreamSinkOutEventBridge {
  type: "eventBridge";
  config: EventBridgeConfig;
}

interface StreamSinkOutSnowflake {
  type: "snowflake";
  config: SnowflakeConfig;
}

interface StreamSinkOutRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfig;
}

interface StreamSinkOutRedshift {
  type: "redshift";
  config: RedshiftConfig;
}

export type StreamSinkOut = _StreamSinkOutFields &
  (
    | StreamSinkOutPoller
    | StreamSinkOutAzureBlobStorage
    | StreamSinkOutOtelTracing
    | StreamSinkOutHttp
    | StreamSinkOutAmazonS3
    | StreamSinkOutGoogleCloudStorage
    | StreamSinkOutGoogleCloudPubSub
    | StreamSinkOutSqs
    | StreamSinkOutSns
    | StreamSinkOutBigQuery
    | StreamSinkOutClickhouse
    | StreamSinkOutEventBridge
    | StreamSinkOutSnowflake
    | StreamSinkOutRabbitMq
    | StreamSinkOutRedshift
  );

export const StreamSinkOutSerializer = {
  _fromJsonObject(object: any): StreamSinkOut {
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
      createdAt: new Date(object["createdAt"]),
      currentIterator: object["currentIterator"],
      eventTypes: object["eventTypes"],
      failureReason: object["failureReason"],
      id: object["id"],
      maxWaitSecs: object["maxWaitSecs"],
      metadata: object["metadata"],
      nextRetryAt: object["nextRetryAt"] ? new Date(object["nextRetryAt"]) : null,
      status: SinkStatusSerializer._fromJsonObject(object["status"]),
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: StreamSinkOut): any {
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
      createdAt: self.createdAt,
      currentIterator: self.currentIterator,
      eventTypes: self.eventTypes,
      failureReason: self.failureReason,
      id: self.id,
      maxWaitSecs: self.maxWaitSecs,
      metadata: self.metadata,
      nextRetryAt: self.nextRetryAt,
      status: SinkStatusSerializer._toJsonObject(self.status),
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
