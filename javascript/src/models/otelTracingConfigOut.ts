// this file is @generated
import {
  type EndpointHeadersOut,
  EndpointHeadersOutSerializer,
} from "./endpointHeadersOut";

export interface OtelTracingConfigOut {
  url: string;
  headers: EndpointHeadersOut;
}

export const OtelTracingConfigOutSerializer = {
  _fromJsonObject(object: any): OtelTracingConfigOut {
    return {
      url: object["url"],
      headers: EndpointHeadersOutSerializer._fromJsonObject(object["headers"]),
    };
  },

  _toJsonObject(self: OtelTracingConfigOut): any {
    return {
      url: self.url,
      headers: EndpointHeadersOutSerializer._toJsonObject(self.headers),
    };
  },
};
