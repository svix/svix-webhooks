// this file is @generated

/** Configuration for a RabbitMq sink. */
export interface RabbitMqConfig {
  uri: string;
  routingKey: string;
}

export const RabbitMqConfigSerializer = {
  _fromJsonObject(object: any): RabbitMqConfig {
    return {
      uri: object["uri"],
      routingKey: object["routingKey"],
    };
  },

  _toJsonObject(self: RabbitMqConfig): any {
    return {
      uri: self.uri,
      routingKey: self.routingKey,
    };
  },
};
