// this file is @generated

export interface GoogleCloudStorageConfigPatch {
  bucket?: string;
  credentials?: string;
}

export const GoogleCloudStorageConfigPatchSerializer = {
  _fromJsonObject(object: any): GoogleCloudStorageConfigPatch {
    return {
      bucket: object["bucket"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudStorageConfigPatch): any {
    return {
      bucket: self.bucket,
      credentials: self.credentials,
    };
  },
};
