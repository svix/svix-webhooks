// this file is @generated

/** Configuration for a SNS sink. */
export interface SnsConfigIn {
  topicArn: string;
  /**
   * The region of the SNS instance.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
  /**
   * Access key ID.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  secretAccessKey?: string | null;
  endpointUrl?: string | null;
}

export const SnsConfigInSerializer = {
  _fromJsonObject(object: any): SnsConfigIn {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SnsConfigIn): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
