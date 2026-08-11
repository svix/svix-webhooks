// this file is @generated

/** Configuration for a RabbitMq sink. */
export interface RabbitMqConfigIn {
  uri: string;
  routingKey: string;
}

export const RabbitMqConfigInSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigIn {
    return {
      uri: object["uri"],
      routingKey: object["routingKey"],
    };
  },

  _toJsonObject(self: RabbitMqConfigIn): any {
    return {
      uri: self.uri,
      routingKey: self.routingKey,
    };
  },
};
