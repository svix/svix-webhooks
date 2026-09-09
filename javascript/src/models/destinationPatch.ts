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
  type DestinationStatusIn,
  DestinationStatusInSerializer,
} from "./destinationStatusIn";
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
  type PostgresConfigPatch,
  PostgresConfigPatchSerializer,
} from "./postgresConfigPatch";
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
import {
  type SnowflakeConfigPatch,
  SnowflakeConfigPatchSerializer,
} from "./snowflakeConfigPatch";
import { type SnsConfigPatch, SnsConfigPatchSerializer } from "./snsConfigPatch";
import { type SqsConfigPatch, SqsConfigPatchSerializer } from "./sqsConfigPatch";

interface _DestinationPatchFields {
  /** The Destination's UID. */
  uid?: string | null;
  status?: DestinationStatusIn | null;
  batchSize?: number | null;
  maxWaitSecs?: number | null;
  eventTypes?: string[];
  channels?: string[];
  metadata?: { [key: string]: string };
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface DestinationPatchPollingEndpointConfig {}

interface DestinationPatchPollingEndpoint {
  type: "pollingEndpoint";
  config?: DestinationPatchPollingEndpointConfig;
}

interface DestinationPatchAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfigPatch;
}

interface DestinationPatchOtelTracing {
  type: "otelTracing";
  config: OtelTracingConfigPatch;
}

interface DestinationPatchFifoEndpoint {
  type: "fifoEndpoint";
  config: SinkHttpConfigPatch;
}

interface DestinationPatchAmazonS3 {
  type: "amazonS3";
  config: S3ConfigPatch;
}

interface DestinationPatchGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfigPatch;
}

interface DestinationPatchGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfigPatch;
}

interface DestinationPatchSqs {
  type: "sqs";
  config: SqsConfigPatch;
}

interface DestinationPatchSns {
  type: "sns";
  config: SnsConfigPatch;
}

interface DestinationPatchBigQuery {
  type: "bigQuery";
  config: BigQueryConfigPatch;
}

interface DestinationPatchClickhouse {
  type: "clickhouse";
  config: ClickhouseConfigPatch;
}

interface DestinationPatchEventBridge {
  type: "eventBridge";
  config: EventBridgeConfigPatch;
}

interface DestinationPatchSnowflake {
  type: "snowflake";
  config: SnowflakeConfigPatch;
}

interface DestinationPatchRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfigPatch;
}

interface DestinationPatchRedshift {
  type: "redshift";
  config: RedshiftConfigPatch;
}

interface DestinationPatchPostgres {
  type: "postgres";
  config: PostgresConfigPatch;
}

export type DestinationPatch = _DestinationPatchFields &
  (
    | DestinationPatchPollingEndpoint
    | DestinationPatchAzureBlobStorage
    | DestinationPatchOtelTracing
    | DestinationPatchFifoEndpoint
    | DestinationPatchAmazonS3
    | DestinationPatchGoogleCloudStorage
    | DestinationPatchGoogleCloudPubSub
    | DestinationPatchSqs
    | DestinationPatchSns
    | DestinationPatchBigQuery
    | DestinationPatchClickhouse
    | DestinationPatchEventBridge
    | DestinationPatchSnowflake
    | DestinationPatchRabbitMq
    | DestinationPatchRedshift
    | DestinationPatchPostgres
  );

export const DestinationPatchSerializer = {
  _fromJsonObject(object: any): DestinationPatch {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "pollingEndpoint":
          return {};
        case "azureBlobStorage":
          return AzureBlobStorageConfigPatchSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingConfigPatchSerializer._fromJsonObject(object["config"]);
        case "fifoEndpoint":
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
        case "postgres":
          return PostgresConfigPatchSerializer._fromJsonObject(object["config"]);
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
          ? DestinationStatusInSerializer._fromJsonObject(object["status"])
          : undefined,
      batchSize: object["batchSize"],
      maxWaitSecs: object["maxWaitSecs"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: DestinationPatch): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "pollingEndpoint":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStorageConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingConfigPatchSerializer._toJsonObject(self.config);
        break;
      case "fifoEndpoint":
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
      case "postgres":
        config = PostgresConfigPatchSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      uid: self.uid,
      status:
        self.status != null
          ? DestinationStatusInSerializer._toJsonObject(self.status)
          : undefined,
      batchSize: self.batchSize,
      maxWaitSecs: self.maxWaitSecs,
      eventTypes: self.eventTypes,
      channels: self.channels,
      metadata: self.metadata,
    };
  },
};
