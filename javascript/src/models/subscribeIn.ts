// this file is @generated
import {
  type AutoConfigSinkType,
  AutoConfigSinkTypeSerializer,
} from "./autoConfigSinkType";
import { type EndpointIn, EndpointInSerializer } from "./endpointIn";

export interface SubscribeIn {
  endpoint?: EndpointIn | null;
  sink?: AutoConfigSinkType | null;
}

export const SubscribeInSerializer = {
  _fromJsonObject(object: any): SubscribeIn {
    return {
      endpoint:
        object["endpoint"] != null
          ? EndpointInSerializer._fromJsonObject(object["endpoint"])
          : undefined,
      sink:
        object["sink"] != null
          ? AutoConfigSinkTypeSerializer._fromJsonObject(object["sink"])
          : undefined,
    };
  },

  _toJsonObject(self: SubscribeIn): any {
    return {
      endpoint:
        self.endpoint != null
          ? EndpointInSerializer._toJsonObject(self.endpoint)
          : undefined,
      sink:
        self.sink != null
          ? AutoConfigSinkTypeSerializer._toJsonObject(self.sink)
          : undefined,
    };
  },
};
