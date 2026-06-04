// this file is @generated

export interface EventBridgeConfig {
  accessKeyId: string;
  /** Free-form string, with a maximum of 128 characters */
  detailType?: string;
  /** The name or ARN of the event bus to receive the event */
  eventBusName: string;
  region: string;
  secretAccessKey: string;
}

export const EventBridgeConfigSerializer = {
  _fromJsonObject(object: any): EventBridgeConfig {
    return {
      accessKeyId: object["accessKeyId"],
      detailType: object["detailType"],
      eventBusName: object["eventBusName"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: EventBridgeConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      detailType: self.detailType,
      eventBusName: self.eventBusName,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
