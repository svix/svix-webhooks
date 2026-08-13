// this file is @generated

export interface S3ConfigIn {
  bucket: string;
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
  /**
   * The region of the EventBridge bus.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
  endpointUrl?: string | null;
}

export const S3ConfigInSerializer = {
  _fromJsonObject(object: any): S3ConfigIn {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: S3ConfigIn): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
