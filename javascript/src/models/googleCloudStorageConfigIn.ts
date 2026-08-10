// this file is @generated

/**
 * Configuration for a Google Cloud Storage sink.
 *
 * Write stream events into the named bucket using the supplied Google Cloud credentials.
 */
export interface GoogleCloudStorageConfigIn {
  bucket: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const GoogleCloudStorageConfigInSerializer = {
  _fromJsonObject(object: any): GoogleCloudStorageConfigIn {
    return {
      bucket: object["bucket"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudStorageConfigIn): any {
    return {
      bucket: self.bucket,
      credentials: self.credentials,
    };
  },
};
