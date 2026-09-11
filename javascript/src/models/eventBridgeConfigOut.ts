// this file is @generated

export interface EventBridgeConfigOut {
  eventBusName: string;
  detailType: string;
  accessKeyId?: string | null;
  roleArn?: string | null;
  externalId?: string | null;
  region: string;
}

export const EventBridgeConfigOutSerializer = {
  _fromJsonObject(object: any): EventBridgeConfigOut {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfigOut): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      roleArn: self.roleArn,
      externalId: self.externalId,
      region: self.region,
    };
  },
};
