// this file is @generated

export interface GenericWebhookResponse {
  contentType: string;
  body: string;
}

export const GenericWebhookResponseSerializer = {
  _fromJsonObject(object: any): GenericWebhookResponse {
    return {
      contentType: object["contentType"],
      body: object["body"],
    };
  },

  _toJsonObject(self: GenericWebhookResponse): any {
    return {
      contentType: self.contentType,
      body: self.body,
    };
  },
};
