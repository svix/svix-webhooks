// this file is @generated
import {
  type EndpointEnabledEventData,
  EndpointEnabledEventDataSerializer,
} from "./endpointEnabledEventData";

/** Sent when an endpoint has been enabled. */
export interface EndpointEnabledEvent {
  data: EndpointEnabledEventData;
  type: string;
}

export const EndpointEnabledEventSerializer = {
  _fromJsonObject(object: any): EndpointEnabledEvent {
    return {
      data: EndpointEnabledEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: EndpointEnabledEvent): any {
    return {
      data: EndpointEnabledEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
