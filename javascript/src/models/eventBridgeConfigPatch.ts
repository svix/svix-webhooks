// this file is @generated

export interface EventBridgeConfigPatch {
  eventBusName?: string;
  detailType?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  roleArn?: string;
  externalId?: string;
  region?: string;
}

export const EventBridgeConfigPatchSerializer = {
  _fromJsonObject(object: any): EventBridgeConfigPatch {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfigPatch): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      roleArn: self.roleArn,
      externalId: self.externalId,
      region: self.region,
    };
  },
};
