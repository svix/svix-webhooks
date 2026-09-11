// this file is @generated

/** Configuration for an SQS sink. */
export interface SqsConfigIn {
  queueUrl: string;
  /**
   * The region of the SQS queue.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
  /**
   * Access key ID.
   *
   * Required (along with `secret_access_key`) if `role_arn` is blank.
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Required (along with `access_key_id`) if `role_arn` is blank.
   */
  secretAccessKey?: string | null;
  /** Role ARN for delegated authentication */
  roleArn?: string | null;
  /**
   * Shared secret passed as the STS ExternalId.
   *
   * Can only be set if `role_arn` is Some
   */
  externalId?: string | null;
  endpointUrl?: string | null;
}

export const SqsConfigInSerializer = {
  _fromJsonObject(object: any): SqsConfigIn {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfigIn): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      roleArn: self.roleArn,
      externalId: self.externalId,
      endpointUrl: self.endpointUrl,
    };
  },
};
