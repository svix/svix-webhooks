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
   * Required (along with `secret_access_key`) if `role_arn` is None
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Required (along with `access_key_id`) if `role_arn` is None
   */
  secretAccessKey?: string | null;
  endpointUrl?: string | null;
  /** Role ARN for delegated authentication */
  roleArn?: string | null;
  /**
   * Shared secret passed as the STS ExternalId.
   *
   * Can only be set if `role_arn` is Some
   */
  externalId?: string | null;
}

export const SnsConfigInSerializer = {
  _fromJsonObject(object: any): SnsConfigIn {
    return {
      topicArn: object["topicArn"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
    };
  },

  _toJsonObject(self: SnsConfigIn): any {
    return {
      topicArn: self.topicArn,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
      roleArn: self.roleArn,
      externalId: self.externalId,
    };
  },
};
