// this file is @generated
import { type EndpointIn, EndpointInSerializer } from "./endpointIn";

export interface SubscribeIn {
  endpoint: EndpointIn;
}

export const SubscribeInSerializer = {
  _fromJsonObject(object: any): SubscribeIn {
    return {
      endpoint: EndpointInSerializer._fromJsonObject(object["endpoint"]),
    };
  },

  _toJsonObject(self: SubscribeIn): any {
    return {
      endpoint: EndpointInSerializer._toJsonObject(self.endpoint),
    };
  },
};
