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

export const SqsConfigInSerializer = {
  _fromJsonObject(object: any): SqsConfigIn {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfigIn): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      endpointUrl: self.endpointUrl,
    };
  },
};
