// this file is @generated
import {
  type AzureBlobStorageConfigOut,
  AzureBlobStorageConfigOutSerializer,
} from "./azureBlobStorageConfigOut";
import { type BigQueryConfigOut, BigQueryConfigOutSerializer } from "./bigQueryConfigOut";
import {
  type ClickhouseConfigOut,
  ClickhouseConfigOutSerializer,
} from "./clickhouseConfigOut";
import {
  type EventBridgeConfigOut,
  EventBridgeConfigOutSerializer,
} from "./eventBridgeConfigOut";
import {
  type GoogleCloudPubSubConfigOut,
  GoogleCloudPubSubConfigOutSerializer,
} from "./googleCloudPubSubConfigOut";
import {
  type GoogleCloudStorageConfigOut,
  GoogleCloudStorageConfigOutSerializer,
} from "./googleCloudStorageConfigOut";
import {
  type OtelTracingConfigOut,
  OtelTracingConfigOutSerializer,
} from "./otelTracingConfigOut";
import { type PostgresConfigOut, PostgresConfigOutSerializer } from "./postgresConfigOut";
import { type RabbitMqConfigOut, RabbitMqConfigOutSerializer } from "./rabbitMqConfigOut";
import { type RedshiftConfigOut, RedshiftConfigOutSerializer } from "./redshiftConfigOut";
import { type S3ConfigOut, S3ConfigOutSerializer } from "./s3ConfigOut";
import { type SinkHttpConfigOut, SinkHttpConfigOutSerializer } from "./sinkHttpConfigOut";
import { type SinkStatus, SinkStatusSerializer } from "./sinkStatus";
import {
  type SnowflakeConfigOut,
  SnowflakeConfigOutSerializer,
} from "./snowflakeConfigOut";
import { type SnsConfigOut, SnsConfigOutSerializer } from "./snsConfigOut";
import { type SqsConfigOut, SqsConfigOutSerializer } from "./sqsConfigOut";

interface _DestinationOutFields {
  /** The destination's ID. */
  id: string;
  /** The destination's UID. */
  uid?: string | null;
  status: SinkStatus;
  currentIterator: string;
  failureReason?: string | null;
  createdAt: Date;
  updatedAt: Date;
  batchSize: number;
  maxWaitSecs: number;
  eventTypes?: string[];
  channels?: string[];
  nextRetryAt?: Date | null;
  metadata: { [key: string]: string };
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface DestinationOutPollingEndpointConfig {}

interface DestinationOutPollingEndpoint {
  type: "pollingEndpoint";
  config?: DestinationOutPollingEndpointConfig;
}

interface DestinationOutAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfigOut;
}

interface DestinationOutOtelTracing {
  type: "otelTracing";
  config: OtelTracingConfigOut;
}

interface DestinationOutFifoEndpoint {
  type: "fifoEndpoint";
  config: SinkHttpConfigOut;
}

interface DestinationOutAmazonS3 {
  type: "amazonS3";
  config: S3ConfigOut;
}

interface DestinationOutSnowflake {
  type: "snowflake";
  config: SnowflakeConfigOut;
}

interface DestinationOutGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfigOut;
}

interface DestinationOutGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfigOut;
}

interface DestinationOutRedshift {
  type: "redshift";
  config: RedshiftConfigOut;
}

interface DestinationOutBigQuery {
  type: "bigQuery";
  config: BigQueryConfigOut;
}

interface DestinationOutClickhouse {
  type: "clickhouse";
  config: ClickhouseConfigOut;
}

interface DestinationOutRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfigOut;
}

interface DestinationOutSqs {
  type: "sqs";
  config: SqsConfigOut;
}

interface DestinationOutEventBridge {
  type: "eventBridge";
  config: EventBridgeConfigOut;
}

interface DestinationOutSns {
  type: "sns";
  config: SnsConfigOut;
}

interface DestinationOutPostgres {
  type: "postgres";
  config: PostgresConfigOut;
}

/** Equivalent to [`SinkConfigOut`], with `fifoEndpoint` instead of `http`. */
export type DestinationOut = _DestinationOutFields &
  (
    | DestinationOutPollingEndpoint
    | DestinationOutAzureBlobStorage
    | DestinationOutOtelTracing
    | DestinationOutFifoEndpoint
    | DestinationOutAmazonS3
    | DestinationOutSnowflake
    | DestinationOutGoogleCloudStorage
    | DestinationOutGoogleCloudPubSub
    | DestinationOutRedshift
    | DestinationOutBigQuery
    | DestinationOutClickhouse
    | DestinationOutRabbitMq
    | DestinationOutSqs
    | DestinationOutEventBridge
    | DestinationOutSns
    | DestinationOutPostgres
  );

export const DestinationOutSerializer = {
  _fromJsonObject(object: any): DestinationOut {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "pollingEndpoint":
          return {};
        case "azureBlobStorage":
          return AzureBlobStorageConfigOutSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingConfigOutSerializer._fromJsonObject(object["config"]);
        case "fifoEndpoint":
          return SinkHttpConfigOutSerializer._fromJsonObject(object["config"]);
        case "amazonS3":
          return S3ConfigOutSerializer._fromJsonObject(object["config"]);
        case "snowflake":
          return SnowflakeConfigOutSerializer._fromJsonObject(object["config"]);
        case "googleCloudStorage":
          return GoogleCloudStorageConfigOutSerializer._fromJsonObject(object["config"]);
        case "googleCloudPubSub":
          return GoogleCloudPubSubConfigOutSerializer._fromJsonObject(object["config"]);
        case "redshift":
          return RedshiftConfigOutSerializer._fromJsonObject(object["config"]);
        case "bigQuery":
          return BigQueryConfigOutSerializer._fromJsonObject(object["config"]);
        case "clickhouse":
          return ClickhouseConfigOutSerializer._fromJsonObject(object["config"]);
        case "rabbitMq":
          return RabbitMqConfigOutSerializer._fromJsonObject(object["config"]);
        case "sqs":
          return SqsConfigOutSerializer._fromJsonObject(object["config"]);
        case "eventBridge":
          return EventBridgeConfigOutSerializer._fromJsonObject(object["config"]);
        case "sns":
          return SnsConfigOutSerializer._fromJsonObject(object["config"]);
        case "postgres":
          return PostgresConfigOutSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }

    return {
      type,
      config: getConfig(type),
      id: object["id"],
      uid: object["uid"],
      status: SinkStatusSerializer._fromJsonObject(object["status"]),
      currentIterator: object["currentIterator"],
      failureReason: object["failureReason"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      batchSize: object["batchSize"],
      maxWaitSecs: object["maxWaitSecs"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      nextRetryAt: object["nextRetryAt"] ? new Date(object["nextRetryAt"]) : null,
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: DestinationOut): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "pollingEndpoint":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStorageConfigOutSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingConfigOutSerializer._toJsonObject(self.config);
        break;
      case "fifoEndpoint":
        config = SinkHttpConfigOutSerializer._toJsonObject(self.config);
        break;
      case "amazonS3":
        config = S3ConfigOutSerializer._toJsonObject(self.config);
        break;
      case "snowflake":
        config = SnowflakeConfigOutSerializer._toJsonObject(self.config);
        break;
      case "googleCloudStorage":
        config = GoogleCloudStorageConfigOutSerializer._toJsonObject(self.config);
        break;
      case "googleCloudPubSub":
        config = GoogleCloudPubSubConfigOutSerializer._toJsonObject(self.config);
        break;
      case "redshift":
        config = RedshiftConfigOutSerializer._toJsonObject(self.config);
        break;
      case "bigQuery":
        config = BigQueryConfigOutSerializer._toJsonObject(self.config);
        break;
      case "clickhouse":
        config = ClickhouseConfigOutSerializer._toJsonObject(self.config);
        break;
      case "rabbitMq":
        config = RabbitMqConfigOutSerializer._toJsonObject(self.config);
        break;
      case "sqs":
        config = SqsConfigOutSerializer._toJsonObject(self.config);
        break;
      case "eventBridge":
        config = EventBridgeConfigOutSerializer._toJsonObject(self.config);
        break;
      case "sns":
        config = SnsConfigOutSerializer._toJsonObject(self.config);
        break;
      case "postgres":
        config = PostgresConfigOutSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
      id: self.id,
      uid: self.uid,
      status: SinkStatusSerializer._toJsonObject(self.status),
      currentIterator: self.currentIterator,
      failureReason: self.failureReason,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      batchSize: self.batchSize,
      maxWaitSecs: self.maxWaitSecs,
      eventTypes: self.eventTypes,
      channels: self.channels,
      nextRetryAt: self.nextRetryAt,
      metadata: self.metadata,
    };
  },
};
