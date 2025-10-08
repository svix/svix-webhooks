// this file is @generated
import {
  type AzureBlobStorageConfig,
  AzureBlobStorageConfigSerializer,
} from "./azureBlobStorageConfig";
import {
  type GoogleCloudStorageConfig,
  GoogleCloudStorageConfigSerializer,
} from "./googleCloudStorageConfig";
import { type S3Config, S3ConfigSerializer } from "./s3Config";
import { type SinkHttpConfig, SinkHttpConfigSerializer } from "./sinkHttpConfig";
import { type SinkOtelV1Config, SinkOtelV1ConfigSerializer } from "./sinkOtelV1Config";
import { type SinkStatus, SinkStatusSerializer } from "./sinkStatus";
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

export type StreamSinkOut = _StreamSinkOutFields &
  (
    | StreamSinkOutPoller
    | StreamSinkOutAzureBlobStorage
    | StreamSinkOutOtelTracing
    | StreamSinkOutHttp
    | StreamSinkOutAmazonS3
    | StreamSinkOutGoogleCloudStorage
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
