// this file is @generated

export interface SnsPatchConfig {
  accessKeyId?: string;
  endpointUrl?: string | null;
  region?: string;
  secretAccessKey?: string;
  topicArn?: string;
}

export const SnsPatchConfigSerializer = {
  _fromJsonObject(object: any): SnsPatchConfig {
    return {
      accessKeyId: object["accessKeyId"],
      endpointUrl: object["endpointUrl"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
      topicArn: object["topicArn"],
    };
  },

  _toJsonObject(self: SnsPatchConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      endpointUrl: self.endpointUrl,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
      topicArn: self.topicArn,
    };
  },
};
