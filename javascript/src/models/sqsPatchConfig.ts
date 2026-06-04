// this file is @generated

export interface SqsPatchConfig {
  accessKeyId?: string;
  endpointUrl?: string | null;
  queueUrl?: string;
  region?: string;
  secretAccessKey?: string;
}

export const SqsPatchConfigSerializer = {
  _fromJsonObject(object: any): SqsPatchConfig {
    return {
      accessKeyId: object["accessKeyId"],
      endpointUrl: object["endpointUrl"],
      queueUrl: object["queueUrl"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: SqsPatchConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      endpointUrl: self.endpointUrl,
      queueUrl: self.queueUrl,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
