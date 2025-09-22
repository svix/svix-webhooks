// this file is @generated

export interface EndpointHeadersPatchIn {
  /** A list of headers be be removed */
  deleteHeaders?: string[];
  headers: { [key: string]: string };
}

export const EndpointHeadersPatchInSerializer = {
  _fromJsonObject(object: any): EndpointHeadersPatchIn {
    return {
      deleteHeaders: object["deleteHeaders"],
      headers: object["headers"],
    };
  },

  _toJsonObject(self: EndpointHeadersPatchIn): any {
    return {
      deleteHeaders: self.deleteHeaders,
      headers: self.headers,
    };
  },
};
