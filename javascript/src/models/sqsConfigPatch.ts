// this file is @generated

export interface SqsConfigPatch {
  queueUrl?: string;
  region?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  endpointUrl?: string | null;
}

export const SqsConfigPatchSerializer = {
  _fromJsonObject(object: any): SqsConfigPatch {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfigPatch): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
