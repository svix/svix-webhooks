// this file is @generated
import {
  type EndpointDisabledTrigger,
  EndpointDisabledTriggerSerializer,
} from "./endpointDisabledTrigger";

/** Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call. */
export interface IngestEndpointDisabledEventData {
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
  failSince?: Date | null;
  /** The Source's ID. */
  sourceId: string;
  trigger?: EndpointDisabledTrigger;
}

export const IngestEndpointDisabledEventDataSerializer = {
  _fromJsonObject(object: any): IngestEndpointDisabledEventData {
    return {
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
      failSince: object["failSince"] ? new Date(object["failSince"]) : null,
      sourceId: object["sourceId"],
      trigger: object["trigger"]
        ? EndpointDisabledTriggerSerializer._fromJsonObject(object["trigger"])
        : undefined,
    };
  },

  _toJsonObject(self: IngestEndpointDisabledEventData): any {
    return {
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
      failSince: self.failSince,
      sourceId: self.sourceId,
      trigger: self.trigger
        ? EndpointDisabledTriggerSerializer._toJsonObject(self.trigger)
        : undefined,
    };
  },
};
