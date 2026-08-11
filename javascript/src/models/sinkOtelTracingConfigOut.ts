// this file is @generated
import {
  type EndpointHeadersOut,
  EndpointHeadersOutSerializer,
} from "./endpointHeadersOut";

export interface SinkOtelTracingConfigOut {
  url: string;
  headers: EndpointHeadersOut;
}

export const SinkOtelTracingConfigOutSerializer = {
  _fromJsonObject(object: any): SinkOtelTracingConfigOut {
    return {
      url: object["url"],
      headers: EndpointHeadersOutSerializer._fromJsonObject(object["headers"]),
    };
  },

  _toJsonObject(self: SinkOtelTracingConfigOut): any {
    return {
      url: self.url,
      headers: EndpointHeadersOutSerializer._toJsonObject(self.headers),
    };
  },
};
