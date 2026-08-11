// this file is @generated

export interface SnsConfigOut {
  topicArn: string;
  region: string;
  accessKeyId: string;
}

export const SnsConfigOutSerializer = {
  _fromJsonObject(object: any): SnsConfigOut {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
    };
  },

  _toJsonObject(self: SnsConfigOut): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
    };
  },
};
