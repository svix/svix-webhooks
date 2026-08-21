// this file is @generated

export interface EventBridgeConfigIn {
  /** The name or ARN of the event bus to receive the event */
  eventBusName: string;
  /** Free-form string, with a maximum of 128 characters */
  detailType?: string;
  /**
   * Access key ID.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  secretAccessKey?: string | null;
  /**
   * The region of the EventBridge bus.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
}

export const EventBridgeConfigInSerializer = {
  _fromJsonObject(object: any): EventBridgeConfigIn {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfigIn): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
    };
  },
};
