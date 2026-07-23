// this file is @generated

/** Configuration for a SNS sink. */
export interface SnsConfig {
  topicArn: string;
  region: string;
  accessKeyId: string;
  secretAccessKey: string;
  endpointUrl?: string | null;
}

export const SnsConfigSerializer = {
  _fromJsonObject(object: any): SnsConfig {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SnsConfig): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
