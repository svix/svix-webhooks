// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EndpointUpdatedEventData,
  EndpointUpdatedEventDataSerializer,
} from "./endpointUpdatedEventData";

/** Sent when an endpoint is updated. */
export interface EndpointUpdatedEvent {
  data: EndpointUpdatedEventData;
  type: string;
}

export const EndpointUpdatedEventSerializer = {
  _fromJsonObject(object: any): EndpointUpdatedEvent {
    return {
      data: EndpointUpdatedEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: EndpointUpdatedEvent): any {
    return {
      data: EndpointUpdatedEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
