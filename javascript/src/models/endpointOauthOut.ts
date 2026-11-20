// this file is @generated
import {
  type EndpointOauthConfigOut,
  EndpointOauthConfigOutSerializer,
} from "./endpointOauthConfigOut";

export interface EndpointOauthOut {
  enabled: boolean;
  config?: EndpointOauthConfigOut | null;
}

export const EndpointOauthOutSerializer = {
  _fromJsonObject(object: any): EndpointOauthOut {
    return {
      enabled: object["enabled"],
      config:
        object["config"] != null
          ? EndpointOauthConfigOutSerializer._fromJsonObject(object["config"])
          : undefined,
    };
  },

  _toJsonObject(self: EndpointOauthOut): any {
    return {
      enabled: self.enabled,
      config:
        self.config != null
          ? EndpointOauthConfigOutSerializer._toJsonObject(self.config)
          : undefined,
    };
  },
};
