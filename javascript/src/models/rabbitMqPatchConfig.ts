// this file is @generated

export interface RabbitMqPatchConfig {
  routingKey?: string;
  uri?: string;
}

export const RabbitMqPatchConfigSerializer = {
  _fromJsonObject(object: any): RabbitMqPatchConfig {
    return {
      routingKey: object["routingKey"],
      uri: object["uri"],
    };
  },

  _toJsonObject(self: RabbitMqPatchConfig): any {
    return {
      routingKey: self.routingKey,
      uri: self.uri,
    };
  },
};
