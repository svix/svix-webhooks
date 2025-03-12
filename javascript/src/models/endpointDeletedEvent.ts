// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EndpointDeletedEventData,
  EndpointDeletedEventDataSerializer,
} from "./endpointDeletedEventData";

/** Sent when an endpoint is deleted. */
export interface EndpointDeletedEvent {
  data: EndpointDeletedEventData;
  type: string;
}

export const EndpointDeletedEventSerializer = {
  _fromJsonObject(object: any): EndpointDeletedEvent {
    return {
      data: EndpointDeletedEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: EndpointDeletedEvent): any {
    return {
      data: EndpointDeletedEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
