// this file is @generated

export interface HttpSinkHeadersPatchIn {
  headers: { [key: string]: string };
}

export const HttpSinkHeadersPatchInSerializer = {
  _fromJsonObject(object: any): HttpSinkHeadersPatchIn {
    return {
      headers: object["headers"],
    };
  },

  _toJsonObject(self: HttpSinkHeadersPatchIn): any {
    return {
      headers: self.headers,
    };
  },
};
