// this file is @generated

export interface S3ConfigIn {
  bucket: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
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
