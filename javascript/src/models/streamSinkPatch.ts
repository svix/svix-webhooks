// this file is @generated
import {
  type AmazonS3PatchConfig,
  AmazonS3PatchConfigSerializer,
} from "./amazonS3PatchConfig";
import {
  type AzureBlobStoragePatchConfig,
  AzureBlobStoragePatchConfigSerializer,
} from "./azureBlobStoragePatchConfig";
import {
  type BigQueryPatchConfig,
  BigQueryPatchConfigSerializer,
} from "./bigQueryPatchConfig";
import {
  type ClickhousePatchConfig,
  ClickhousePatchConfigSerializer,
} from "./clickhousePatchConfig";
import {
  type EventBridgePatchConfig,
  EventBridgePatchConfigSerializer,
} from "./eventBridgePatchConfig";
import {
  type GoogleCloudPubSubPatchConfig,
  GoogleCloudPubSubPatchConfigSerializer,
} from "./googleCloudPubSubPatchConfig";
import {
  type GoogleCloudStoragePatchConfig,
  GoogleCloudStoragePatchConfigSerializer,
} from "./googleCloudStoragePatchConfig";
import { type HttpPatchConfig, HttpPatchConfigSerializer } from "./httpPatchConfig";
import {
  type OtelTracingPatchConfig,
  OtelTracingPatchConfigSerializer,
} from "./otelTracingPatchConfig";
import {
  type RabbitMqPatchConfig,
  RabbitMqPatchConfigSerializer,
} from "./rabbitMqPatchConfig";
import {
  type RedshiftPatchConfig,
  RedshiftPatchConfigSerializer,
} from "./redshiftPatchConfig";
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
import {
  type SnowflakePatchConfig,
  SnowflakePatchConfigSerializer,
} from "./snowflakePatchConfig";
import { type SnsPatchConfig, SnsPatchConfigSerializer } from "./snsPatchConfig";
import { type SqsPatchConfig, SqsPatchConfigSerializer } from "./sqsPatchConfig";

interface _StreamSinkPatchFields {
  batchSize?: number | null;
  eventTypes?: string[];
  maxWaitSecs?: number | null;
  metadata?: { [key: string]: string };
  status?: SinkStatusIn | null;
  /** The StreamSink's UID. */
  uid?: string | null;
}

// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface StreamSinkPatchPollerConfig {}

interface StreamSinkPatchPoller {
  type: "poller";
  config?: StreamSinkPatchPollerConfig;
}

interface StreamSinkPatchAzureBlobStorage {
  type: "azureBlobStorage";
  config: AzureBlobStoragePatchConfig;
}

interface StreamSinkPatchOtelTracing {
  type: "otelTracing";
  config: OtelTracingPatchConfig;
}

interface StreamSinkPatchHttp {
  type: "http";
  config: HttpPatchConfig;
}

interface StreamSinkPatchAmazonS3 {
  type: "amazonS3";
  config: AmazonS3PatchConfig;
}

interface StreamSinkPatchGoogleCloudStorage {
  type: "googleCloudStorage";
  config: GoogleCloudStoragePatchConfig;
}

interface StreamSinkPatchGoogleCloudPubSub {
  type: "googleCloudPubSub";
  config: GoogleCloudPubSubPatchConfig;
}

interface StreamSinkPatchSqs {
  type: "sqs";
  config: SqsPatchConfig;
}

interface StreamSinkPatchSns {
  type: "sns";
  config: SnsPatchConfig;
}

interface StreamSinkPatchBigQuery {
  type: "bigQuery";
  config: BigQueryPatchConfig;
}

interface StreamSinkPatchClickhouse {
  type: "clickhouse";
  config: ClickhousePatchConfig;
}

interface StreamSinkPatchEventBridge {
  type: "eventBridge";
  config: EventBridgePatchConfig;
}

interface StreamSinkPatchSnowflake {
  type: "snowflake";
  config: SnowflakePatchConfig;
}

interface StreamSinkPatchRabbitMq {
  type: "rabbitMq";
  config: RabbitMqPatchConfig;
}

interface StreamSinkPatchRedshift {
  type: "redshift";
  config: RedshiftPatchConfig;
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
          return AzureBlobStoragePatchConfigSerializer._fromJsonObject(object["config"]);
        case "otelTracing":
          return OtelTracingPatchConfigSerializer._fromJsonObject(object["config"]);
        case "http":
          return HttpPatchConfigSerializer._fromJsonObject(object["config"]);
        case "amazonS3":
          return AmazonS3PatchConfigSerializer._fromJsonObject(object["config"]);
        case "googleCloudStorage":
          return GoogleCloudStoragePatchConfigSerializer._fromJsonObject(
            object["config"]
          );
        case "googleCloudPubSub":
          return GoogleCloudPubSubPatchConfigSerializer._fromJsonObject(object["config"]);
        case "sqs":
          return SqsPatchConfigSerializer._fromJsonObject(object["config"]);
        case "sns":
          return SnsPatchConfigSerializer._fromJsonObject(object["config"]);
        case "bigQuery":
          return BigQueryPatchConfigSerializer._fromJsonObject(object["config"]);
        case "clickhouse":
          return ClickhousePatchConfigSerializer._fromJsonObject(object["config"]);
        case "eventBridge":
          return EventBridgePatchConfigSerializer._fromJsonObject(object["config"]);
        case "snowflake":
          return SnowflakePatchConfigSerializer._fromJsonObject(object["config"]);
        case "rabbitMq":
          return RabbitMqPatchConfigSerializer._fromJsonObject(object["config"]);
        case "redshift":
          return RedshiftPatchConfigSerializer._fromJsonObject(object["config"]);
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

  _toJsonObject(self: StreamSinkPatch): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "poller":
        config = {};
        break;
      case "azureBlobStorage":
        config = AzureBlobStoragePatchConfigSerializer._toJsonObject(self.config);
        break;
      case "otelTracing":
        config = OtelTracingPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "http":
        config = HttpPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "amazonS3":
        config = AmazonS3PatchConfigSerializer._toJsonObject(self.config);
        break;
      case "googleCloudStorage":
        config = GoogleCloudStoragePatchConfigSerializer._toJsonObject(self.config);
        break;
      case "googleCloudPubSub":
        config = GoogleCloudPubSubPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "sqs":
        config = SqsPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "sns":
        config = SnsPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "bigQuery":
        config = BigQueryPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "clickhouse":
        config = ClickhousePatchConfigSerializer._toJsonObject(self.config);
        break;
      case "eventBridge":
        config = EventBridgePatchConfigSerializer._toJsonObject(self.config);
        break;
      case "snowflake":
        config = SnowflakePatchConfigSerializer._toJsonObject(self.config);
        break;
      case "rabbitMq":
        config = RabbitMqPatchConfigSerializer._toJsonObject(self.config);
        break;
      case "redshift":
        config = RedshiftPatchConfigSerializer._toJsonObject(self.config);
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
