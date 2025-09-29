// this file is @generated
import {
  type EndpointDisabledEventData,
  EndpointDisabledEventDataSerializer,
} from "./endpointDisabledEventData";

/** Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call. */
export interface EndpointDisabledEvent {
  data: EndpointDisabledEventData;
  type: string;
}

export const EndpointDisabledEventSerializer = {
  _fromJsonObject(object: any): EndpointDisabledEvent {
    return {
      data: EndpointDisabledEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: EndpointDisabledEvent): any {
    return {
      data: EndpointDisabledEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
