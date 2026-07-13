// this file is @generated

export interface EndpointHeadersPatchIn {
  headers: { [key: string]: string };
  /** A list of headers be be removed */
  deleteHeaders?: string[];
}

export const EndpointHeadersPatchInSerializer = {
  _fromJsonObject(object: any): EndpointHeadersPatchIn {
    return {
      headers: object["headers"],
      deleteHeaders: object["deleteHeaders"],
    };
  },

  _toJsonObject(self: EndpointHeadersPatchIn): any {
    return {
      headers: self.headers,
      deleteHeaders: self.deleteHeaders,
    };
  },
};
