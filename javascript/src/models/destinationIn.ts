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
  type FifoEndpointConfigIn,
  FifoEndpointConfigInSerializer,
} from "./fifoEndpointConfigIn";
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
import { type PostgresConfigIn, PostgresConfigInSerializer } from "./postgresConfigIn";
import { type RabbitMqConfigIn, RabbitMqConfigInSerializer } from "./rabbitMqConfigIn";
import { type RedshiftConfigIn, RedshiftConfigInSerializer } from "./redshiftConfigIn";
import { type S3ConfigIn, S3ConfigInSerializer } from "./s3ConfigIn";
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
import { type SnowflakeConfigIn, SnowflakeConfigInSerializer } from "./snowflakeConfigIn";
import { type SnsConfigIn, SnsConfigInSerializer } from "./snsConfigIn";
import { type SqsConfigIn, SqsConfigInSerializer } from "./sqsConfigIn";

interface _DestinationInFields {
  /** An optional unique identifier for the destination. */
  uid?: string | null;
  /**
   * Whether the destination will receive events.
   *
   * If the destination is `enabled`, events sent to the application will be dispatched to the destination in order.
   *
   * If the destination is `disabled`, events will not be dispatched until the destination is reenabled.
   */
  status?: SinkStatusIn;
  /** How many events will be batched in a request to the destination. */
  batchSize?: number;
  /**
   * How long to wait before a batch of events is sent, if the `batchSize` is not reached.
   *
   * For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a request is sent after 10 seconds or 100 events, whichever comes first.
   *
   * Note that an empty batch is never sent to the destination.
   */
  maxWaitSecs?: number;
  /** A list of event types that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events. */
  eventTypes?: string[];
  /** A list of channels that filter which events are dispatched to the destination. An empty list (or null) will not filter out any events. */
  channels?: string[];
  metadata?: { [key: string]: string };
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface DestinationInPollingEndpointConfig {}

interface DestinationInPollingEndpoint {
  type: "pollingEndpoint";
  config?: DestinationInPollingEndpointConfig;
}

interface DestinationInAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStorageConfigIn;
}

interface DestinationInOtelTracing {
  type: "otelTracing";
  config: OtelTracingConfigIn;
}

interface DestinationInFifoEndpoint {
  type: "fifoEndpoint";
  config: FifoEndpointConfigIn;
}

interface DestinationInAmazonS3 {
  type: "amazonS3";
  config: S3ConfigIn;
}

interface DestinationInGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStorageConfigIn;
}

interface DestinationInGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubConfigIn;
}

interface DestinationInSqs {
  type: "sqs";
  config: SqsConfigIn;
}

interface DestinationInSns {
  type: "sns";
  config: SnsConfigIn;
}

interface DestinationInBigQuery {
  type: "bigQuery";
  config: BigQueryConfigIn;
}

interface DestinationInClickhouse {
  type: "clickhouse";
  config: ClickhouseConfigIn;
}

interface DestinationInEventBridge {
  type: "eventBridge";
  config: EventBridgeConfigIn;
}

interface DestinationInSnowflake {
  type: "snowflake";
  config: SnowflakeConfigIn;
}

interface DestinationInRabbitMq {
  type: "rabbitMq";
  config: RabbitMqConfigIn;
}

interface DestinationInRedshift {
  type: "redshift";
  config: RedshiftConfigIn;
}

interface DestinationInPostgres {
  type: "postgres";
  config: PostgresConfigIn;
}

/** The destination's type and type-specific configuration. */
export type DestinationIn = _DestinationInFields &
  (
    | DestinationInPollingEndpoint
    | DestinationInAzureBlobStorage
    | DestinationInOtelTracing
    | DestinationInFifoEndpoint
    | DestinationInAmazonS3
    | DestinationInGoogleCloudStorage
    | DestinationInGoogleCloudPubSub
    | DestinationInSqs
    | DestinationInSns
    | DestinationInBigQuery
    | DestinationInClickhouse
    | DestinationInEventBridge
    | DestinationInSnowflake
    | DestinationInRabbitMq
    | DestinationInRedshift
    | DestinationInPostgres
  );

export const DestinationInSerializer = {
  _fromJsonObject(object: any): DestinationIn {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "pollingEndpoint":
          return {};
        case "azureBlobStorage":
          return AzureBlobStorageConfigInSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingConfigInSerializer._fromJsonObject(object["config"]);
        case "fifoEndpoint":
          return FifoEndpointConfigInSerializer._fromJsonObject(object["config"]);
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
        case "postgres":
          return PostgresConfigInSerializer._fromJsonObject(object["config"]);
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

  _toJsonObject(self: DestinationIn): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "pollingEndpoint":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStorageConfigInSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingConfigInSerializer._toJsonObject(self.config);
        break;
      case "fifoEndpoint":
        config = FifoEndpointConfigInSerializer._toJsonObject(self.config);
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
      case "postgres":
        config = PostgresConfigInSerializer._toJsonObject(self.config);
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
