// this file is @generated

export interface S3Config {
  bucket: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
  endpointUrl?: string | null;
}

export const S3ConfigSerializer = {
  _fromJsonObject(object: any): S3Config {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: S3Config): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
