// this file is @generated
import {
  type IngestEndpointDisabledEventData,
  IngestEndpointDisabledEventDataSerializer,
} from "./ingestEndpointDisabledEventData";

/** Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call. */
export interface IngestEndpointDisabledEvent {
  data: IngestEndpointDisabledEventData;
  type: string;
}

export const IngestEndpointDisabledEventSerializer = {
  _fromJsonObject(object: any): IngestEndpointDisabledEvent {
    return {
      data: IngestEndpointDisabledEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: IngestEndpointDisabledEvent): any {
    return {
      data: IngestEndpointDisabledEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
