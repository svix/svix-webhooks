// this file is @generated
import {
  type EndpointCreatedEventData,
  EndpointCreatedEventDataSerializer,
} from "./endpointCreatedEventData";

/** Sent when an endpoint is created. */
export interface EndpointCreatedEvent {
  data: EndpointCreatedEventData;
  type: string;
}

export const EndpointCreatedEventSerializer = {
  _fromJsonObject(object: any): EndpointCreatedEvent {
    return {
      data: EndpointCreatedEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: EndpointCreatedEvent): any {
    return {
      data: EndpointCreatedEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
