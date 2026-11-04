// this file is @generated

export interface RabbitMqConfigOut {
  uri: string;
  routingKey: string;
  mandatory: boolean;
}

export const RabbitMqConfigOutSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigOut {
    return {
      uri: object["uri"],
      routingKey: object["routingKey"],
      mandatory: object["mandatory"],
    };
  },

  _toJsonObject(self: RabbitMqConfigOut): any {
    return {
      uri: self.uri,
      routingKey: self.routingKey,
      mandatory: self.mandatory,
    };
  },
};
