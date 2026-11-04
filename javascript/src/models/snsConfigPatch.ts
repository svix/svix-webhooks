// this file is @generated

export interface SnsConfigPatch {
  topicArn?: string;
  region?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  roleArn?: string;
  externalId?: string;
  endpointUrl?: string | null;
}

export const SnsConfigPatchSerializer = {
  _fromJsonObject(object: any): SnsConfigPatch {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SnsConfigPatch): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      roleArn: self.roleArn,
      externalId: self.externalId,
      endpointUrl: self.endpointUrl,
    };
  },
};
