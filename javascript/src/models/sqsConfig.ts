// this file is @generated

/** Configuration for an SQS sink. */
export interface SqsConfig {
  queueUrl: string;
  region: string;
  accessKeyId: string;
  secretAccessKey: string;
  endpointUrl?: string | null;
}

export const SqsConfigSerializer = {
  _fromJsonObject(object: any): SqsConfig {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfig): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
