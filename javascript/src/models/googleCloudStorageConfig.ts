// this file is @generated

/**
 * Configuration for a Google Cloud Storage sink.
 *
 * Write stream events into the named bucket using the supplied Google Cloud credentials.
 */
export interface GoogleCloudStorageConfig {
  bucket: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const GoogleCloudStorageConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudStorageConfig {
    return {
      bucket: object["bucket"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudStorageConfig): any {
    return {
      bucket: self.bucket,
      credentials: self.credentials,
    };
  },
};
