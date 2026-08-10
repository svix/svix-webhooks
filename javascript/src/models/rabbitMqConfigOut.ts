// this file is @generated

export interface RabbitMqConfigOut {
  routingKey: string;
}

export const RabbitMqConfigOutSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigOut {
    return {
      routingKey: object["routingKey"],
    };
  },

  _toJsonObject(self: RabbitMqConfigOut): any {
    return {
      routingKey: self.routingKey,
    };
  },
};
