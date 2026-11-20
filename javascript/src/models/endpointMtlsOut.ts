// this file is @generated
import {
  type EndpointMtlsConfigOut,
  EndpointMtlsConfigOutSerializer,
} from "./endpointMtlsConfigOut";

export interface EndpointMtlsOut {
  enabled: boolean;
  config?: EndpointMtlsConfigOut | null;
}

export const EndpointMtlsOutSerializer = {
  _fromJsonObject(object: any): EndpointMtlsOut {
    return {
      enabled: object["enabled"],
      config:
        object["config"] != null
          ? EndpointMtlsConfigOutSerializer._fromJsonObject(object["config"])
          : undefined,
    };
  },

  _toJsonObject(self: EndpointMtlsOut): any {
    return {
      enabled: self.enabled,
      config:
        self.config != null
          ? EndpointMtlsConfigOutSerializer._toJsonObject(self.config)
          : undefined,
    };
  },
};
