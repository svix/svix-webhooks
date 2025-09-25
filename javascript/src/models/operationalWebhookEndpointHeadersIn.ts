// this file is @generated

export interface OperationalWebhookEndpointHeadersIn {
  headers: { [key: string]: string };
}

export const OperationalWebhookEndpointHeadersInSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointHeadersIn {
    return {
      headers: object["headers"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointHeadersIn): any {
    return {
      headers: self.headers,
    };
  },
};
