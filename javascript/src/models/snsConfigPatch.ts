// this file is @generated

export interface SnsConfigPatch {
  topicArn?: string;
  region?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  endpointUrl?: string | null;
}

export const SnsConfigPatchSerializer = {
  _fromJsonObject(object: any): SnsConfigPatch {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SnsConfigPatch): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
