// this file is @generated
import {
  type AzureBlobStorageConfigPatch,
  AzureBlobStorageConfigPatchSerializer,
} from "./azureBlobStorageConfigPatch";
import {
  type BigQueryConfigPatch,
  BigQueryConfigPatchSerializer,
} from "./bigQueryConfigPatch";
import {
  type ClickhouseConfigPatch,
  ClickhouseConfigPatchSerializer,
} from "./clickhouseConfigPatch";
import {
  type EventBridgeConfigPatch,
  EventBridgeConfigPatchSerializer,
} from "./eventBridgeConfigPatch";
import {
  type GoogleCloudPubSubConfigPatch,
  GoogleCloudPubSubConfigPatchSerializer,
} from "./googleCloudPubSubConfigPatch";
import {
  type GoogleCloudStorageConfigPatch,
  GoogleCloudStorageConfigPatchSerializer,
} from "./googleCloudStorageConfigPatch";
import {
  type OtelTracingConfigPatch,
  OtelTracingConfigPatchSerializer,
} from "./otelTracingConfigPatch";
import {
  type RabbitMqConfigPatch,
  RabbitMqConfigPatchSerializer,
} from "./rabbitMqConfigPatch";
import {
  type RedshiftConfigPatch,
  RedshiftConfigPatchSerializer,
} from "./redshiftConfigPatch";
import { type S3ConfigPatch, S3ConfigPatchSerializer } from "./s3ConfigPatch";
import {
  type SinkHttpConfigPatch,
  SinkHttpConfigPatchSerializer,
} from "./sinkHttpConfigPatch";
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
import {
  type SnowflakeConfigPatch,
  SnowflakeConfigPatchSerializer,
} from "./snowflakeConfigPatch";
import { type SnsConfigPatch, SnsConfigPatchSerializer } from "./snsConfigPatch";
import { type SqsConfigPatch, SqsConfigPatchSerializer } from "./sqsConfigPatch";

interface _StreamSinkPatchFields {
  /** The StreamSink's UID. */
  uid?: string | null;
  status?: SinkStatusIn | null;
  batchSize?: number | null;
  maxWaitSecs?: number | null;
  eventTypes?: string[];
  channels?: string[];
  metadata?: { [key: string]: string };
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface StreamSinkPatchPollerConfig {}

interface StreamSinkPatchPoller {
  type: "poller";
  config?: StreamSinkPatchPollerConfig;
}

interface StreamSinkPatchAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfigPatch;
}

interface StreamSinkPatchOtelTracing {
  type: "otelTracing";
  config: OtelTracingConfigPatch;
}

interface StreamSinkPatchHttp {
  type: "http";
  config: SinkHttpConfigPatch;
}

interface StreamSinkPatchAmazonS3 {
  type: "amazonS3";
  config: S3ConfigPatch;
}

interface StreamSinkPatchGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfigPatch;
}

interface StreamSinkPatchGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfigPatch;
}

interface StreamSinkPatchSqs {
  type: "sqs";
  config: SqsConfigPatch;
}

interface StreamSinkPatchSns {
  type: "sns";
  config: SnsConfigPatch;
}

interface StreamSinkPatchBigQuery {
  type: "bigQuery";
  config: BigQueryConfigPatch;
}

interface StreamSinkPatchClickhouse {
  type: "clickhouse";
  config: ClickhouseConfigPatch;
}

interface StreamSinkPatchEventBridge {
  type: "eventBridge";
  config: EventBridgeConfigPatch;
}

interface StreamSinkPatchSnowflake {
  type: "snowflake";
  config: SnowflakeConfigPatch;
}

interface StreamSinkPatchRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfigPatch;
}

interface StreamSinkPatchRedshift {
  type: "redshift";
  config: RedshiftConfigPatch;
}

export type StreamSinkPatch = _StreamSinkPatchFields &
  (
    | StreamSinkPatchPoller
    | StreamSinkPatchAzureBlobStorage
    | StreamSinkPatchOtelTracing
    | StreamSinkPatchHttp
    | StreamSinkPatchAmazonS3
    | StreamSinkPatchGoogleCloudStorage
    | StreamSinkPatchGoogleCloudPubSub
    | StreamSinkPatchSqs
    | StreamSinkPatchSns
    | StreamSinkPatchBigQuery
    | StreamSinkPatchClickhouse
    | StreamSinkPatchEventBridge
    | StreamSinkPatchSnowflake
    | StreamSinkPatchRabbitMq
    | StreamSinkPatchRedshift
  );

export const StreamSinkPatchSerializer = {
  _fromJsonObject(object: any): StreamSinkPatch {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "poller":
          return {};
        case "azureBlobStorage":
          return AzureBlobStorageConfigPatchSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingConfigPatchSerializer._fromJsonObject(object["config"]);
        case "http":
          return SinkHttpConfigPatchSerializer._fromJsonObject(object["config"]);
        case "amazonS3":
          return S3ConfigPatchSerializer._fromJsonObject(object["config"]);
        case "googleCloudStorage":
          return GoogleCloudStorageConfigPatchSerializer._fromJsonObject(
            object["config"]
          );
        case "googleCloudPubSub":
          return GoogleCloudPubSubConfigPatchSerializer._fromJsonObject(object["config"]);
        case "sqs":
          return SqsConfigPatchSerializer._fromJsonObject(object["config"]);
        case "sns":
          return SnsConfigPatchSerializer._fromJsonObject(object["config"]);
        case "bigQuery":
          return BigQueryConfigPatchSerializer._fromJsonObject(object["config"]);
        case "clickhouse":
          return ClickhouseConfigPatchSerializer._fromJsonObject(object["config"]);
        case "eventBridge":
          return EventBridgeConfigPatchSerializer._fromJsonObject(object["config"]);
        case "snowflake":
          return SnowflakeConfigPatchSerializer._fromJsonObject(object["config"]);
        case "rabbitMq":
          return RabbitMqConfigPatchSerializer._fromJsonObject(object["config"]);
        case "redshift":
          return RedshiftConfigPatchSerializer._fromJsonObject(object["config"]);
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

  _toJsonObject(self: StreamSinkPatch): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "poller":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStorageConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "http":
        config = SinkHttpConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "amazonS3":
        config = S3ConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "googleCloudStorage":
        config = GoogleCloudStorageConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "googleCloudPubSub":
        config = GoogleCloudPubSubConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "sqs":
        config = SqsConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "sns":
        config = SnsConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "bigQuery":
        config = BigQueryConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "clickhouse":
        config = ClickhouseConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "eventBridge":
        config = EventBridgeConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "snowflake":
        config = SnowflakeConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "rabbitMq":
        config = RabbitMqConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "redshift":
        config = RedshiftConfigPatchSerializer._toJsonObject(self.config);
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
