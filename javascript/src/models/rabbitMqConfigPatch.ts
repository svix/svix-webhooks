// this file is @generated

export interface RabbitMqConfigPatch {
  routingKey?: string;
  uri?: string;
}

export const RabbitMqConfigPatchSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigPatch {
    return {
      routingKey: object["routingKey"],
      uri: object["uri"],
    };
  },

  _toJsonObject(self: RabbitMqConfigPatch): any {
    return {
      routingKey: self.routingKey,
      uri: self.uri,
    };
  },
};
