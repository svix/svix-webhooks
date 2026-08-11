// this file is @generated
import {
  type EndpointHeadersOut,
  EndpointHeadersOutSerializer,
} from "./endpointHeadersOut";

export interface SinkHttpConfigOut {
  url: string;
  headers: EndpointHeadersOut;
}

export const SinkHttpConfigOutSerializer = {
  _fromJsonObject(object: any): SinkHttpConfigOut {
    return {
      url: object["url"],
      headers: EndpointHeadersOutSerializer._fromJsonObject(object["headers"]),
    };
  },

  _toJsonObject(self: SinkHttpConfigOut): any {
    return {
      url: self.url,
      headers: EndpointHeadersOutSerializer._toJsonObject(self.headers),
    };
  },
};
