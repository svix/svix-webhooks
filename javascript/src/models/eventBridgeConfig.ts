// this file is @generated

export interface EventBridgeConfig {
  /** The name or ARN of the event bus to receive the event */
  eventBusName: string;
  /** Free-form string, with a maximum of 128 characters */
  detailType?: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
}

export const EventBridgeConfigSerializer = {
  _fromJsonObject(object: any): EventBridgeConfig {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfig): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
    };
  },
};
