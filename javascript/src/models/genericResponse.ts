// this file is @generated

export interface GenericResponse {
  contentType: string;
  body: string;
}

export const GenericResponseSerializer = {
  _fromJsonObject(object: any): GenericResponse {
    return {
      contentType: object["contentType"],
      body: object["body"],
    };
  },

  _toJsonObject(self: GenericResponse): any {
    return {
      contentType: self.contentType,
      body: self.body,
    };
  },
};
