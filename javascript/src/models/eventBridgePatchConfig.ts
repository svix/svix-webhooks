// this file is @generated

export interface EventBridgePatchConfig {
  accessKeyId?: string;
  detailType?: string;
  eventBusName?: string;
  region?: string;
  secretAccessKey?: string;
}

export const EventBridgePatchConfigSerializer = {
  _fromJsonObject(object: any): EventBridgePatchConfig {
    return {
      accessKeyId: object["accessKeyId"],
      detailType: object["detailType"],
      eventBusName: object["eventBusName"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: EventBridgePatchConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      detailType: self.detailType,
      eventBusName: self.eventBusName,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
