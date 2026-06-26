// this file is @generated

/** Configuration for an SQS sink. */
export interface SqsConfig {
  accessKeyId: string;
  endpointUrl?: string | null;
  queueUrl: string;
  region: string;
  secretAccessKey: string;
}

export const SqsConfigSerializer = {
  _fromJsonObject(object: any): SqsConfig {
    return {
      accessKeyId: object["accessKeyId"],
      endpointUrl: object["endpointUrl"],
      queueUrl: object["queueUrl"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: SqsConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      endpointUrl: self.endpointUrl,
      queueUrl: self.queueUrl,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
