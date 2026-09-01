// this file is @generated

export interface S3ConfigIn {
  bucket: string;
  /**
   * Access key ID.
   *
   * Required (along with `secret_access_key`) if `role_arn` is null
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Required (along with `access_key_id`) if `role_arn` is null
   */
  secretAccessKey?: string | null;
  /**
   * The region of the S3 bucket
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
  endpointUrl?: string | null;
  /** Role ARN for delegated authentication */
  roleArn?: string | null;
  /**
   * Shared secret passed as the STS ExternalId.
   *
   * Recommended if `role_arn` is not null.
   */
  externalId?: string | null;
}

export const S3ConfigInSerializer = {
  _fromJsonObject(object: any): S3ConfigIn {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
    };
  },

  _toJsonObject(self: S3ConfigIn): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
      roleArn: self.roleArn,
      externalId: self.externalId,
    };
  },
};
