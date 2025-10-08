// this file is @generated

export interface GoogleCloudStoragePatchConfig {
  bucket?: string;
  credentials?: string;
}

export const GoogleCloudStoragePatchConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudStoragePatchConfig {
    return {
      bucket: object["bucket"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudStoragePatchConfig): any {
    return {
      bucket: self.bucket,
      credentials: self.credentials,
    };
  },
};
