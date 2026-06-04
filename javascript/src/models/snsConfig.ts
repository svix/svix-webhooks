// this file is @generated

/** Configuration for a SNS sink. */
export interface SnsConfig {
  accessKeyId: string;
  endpointUrl?: string | null;
  region: string;
  secretAccessKey: string;
  topicArn: string;
}

export const SnsConfigSerializer = {
  _fromJsonObject(object: any): SnsConfig {
    return {
      accessKeyId: object["accessKeyId"],
      endpointUrl: object["endpointUrl"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
      topicArn: object["topicArn"],
    };
  },

  _toJsonObject(self: SnsConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      endpointUrl: self.endpointUrl,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
      topicArn: self.topicArn,
    };
  },
};
