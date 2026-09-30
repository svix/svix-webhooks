// this file is @generated

/** Configuration for a RabbitMq sink. */
export interface RabbitMqConfigIn {
  /**
   * URI to connect to
   *
   * Note that the VHost must be percent-escaped, so a default URI would look
   * like `amqp://user:pass@host/%2F`
   */
  uri: string;
  /** Routing key for message dispatch */
  routingKey: string;
  /**
   * If true, then dispatches will fail if there is no attached queue; if false, they are
   * silently dropped (this was previously the default)
   */
  mandatory?: boolean;
}

export const RabbitMqConfigInSerializer = {
  _fromJsonObject(object: any): RabbitMqConfigIn {
    return {
      uri: object["uri"],
      routingKey: object["routingKey"],
      mandatory: object["mandatory"],
    };
  },

  _toJsonObject(self: RabbitMqConfigIn): any {
    return {
      uri: self.uri,
      routingKey: self.routingKey,
      mandatory: self.mandatory,
    };
  },
};
