// this file is @generated

export interface GoogleCloudStorageConfigOut {
  bucket: string;
}

export const GoogleCloudStorageConfigOutSerializer = {
  _fromJsonObject(object: any): GoogleCloudStorageConfigOut {
    return {
      bucket: object["bucket"],
    };
  },

  _toJsonObject(self: GoogleCloudStorageConfigOut): any {
    return {
      bucket: self.bucket,
    };
  },
};
