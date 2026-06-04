// this file is @generated

/** Configuration for a RabbitMq sink. */
export interface RabbitMqConfig {
  routingKey: string;
  uri: string;
}

export const RabbitMqConfigSerializer = {
  _fromJsonObject(object: any): RabbitMqConfig {
    return {
      routingKey: object["routingKey"],
      uri: object["uri"],
    };
  },

  _toJsonObject(self: RabbitMqConfig): any {
    return {
      routingKey: self.routingKey,
      uri: self.uri,
    };
  },
};
