// this file is @generated

export interface EventBridgeConfigOut {
  eventBusName: string;
  detailType: string;
  accessKeyId: string;
  region: string;
}

export const EventBridgeConfigOutSerializer = {
  _fromJsonObject(object: any): EventBridgeConfigOut {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfigOut): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      region: self.region,
    };
  },
};
