// this file is @generated

export interface SnsConfigOut {
  topicArn: string;
  region: string;
  accessKeyId?: string | null;
  roleArn?: string | null;
  externalId?: string | null;
}

export const SnsConfigOutSerializer = {
  _fromJsonObject(object: any): SnsConfigOut {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
    };
  },

  _toJsonObject(self: SnsConfigOut): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      roleArn: self.roleArn,
      externalId: self.externalId,
    };
  },
};
