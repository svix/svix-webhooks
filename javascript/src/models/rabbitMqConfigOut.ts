// this file is @generated

export interface RabbitMqConfigOut {
  uri: string;
  routingKey: string;
}

export const RabbitMqConfigOutSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigOut {
    return {
      uri: object["uri"],
      routingKey: object["routingKey"],
    };
  },

  _toJsonObject(self: RabbitMqConfigOut): any {
    return {
      uri: self.uri,
      routingKey: self.routingKey,
    };
  },
};
