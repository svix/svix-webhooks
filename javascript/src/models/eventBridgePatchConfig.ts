// this file is @generated

export interface EventBridgePatchConfig {
  eventBusName?: string;
  detailType?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
}

export const EventBridgePatchConfigSerializer = {
  _fromJsonObject(object: any): EventBridgePatchConfig {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgePatchConfig): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
    };
  },
};
