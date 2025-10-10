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
import { type SinkStatusIn, SinkStatusInSerializer } from "./sinkStatusIn";
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

export type StreamSinkIn = _StreamSinkInFields &
  (
    | StreamSinkInPoller
    | StreamSinkInAzureBlobStorage
    | StreamSinkInOtelTracing
    | StreamSinkInHttp
    | StreamSinkInAmazonS3
    | StreamSinkInGoogleCloudStorage
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
      status: object["status"]
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
    }

    return {
      type: self.type,
      config: config,
      batchSize: self.batchSize,
      eventTypes: self.eventTypes,
      maxWaitSecs: self.maxWaitSecs,
      metadata: self.metadata,
      status: self.status ? SinkStatusInSerializer._toJsonObject(self.status) : undefined,
      uid: self.uid,
    };
  },
};
