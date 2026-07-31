// this file is @generated

export interface SqsPatchConfig {
  queueUrl?: string;
  region?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  endpointUrl?: string | null;
}

export const SqsPatchConfigSerializer = {
  _fromJsonObject(object: any): SqsPatchConfig {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsPatchConfig): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
