// this file is @generated

export interface S3Config {
  accessKeyId: string;
  bucket: string;
  endpointUrl?: string | null;
  region: string;
  secretAccessKey: string;
}

export const S3ConfigSerializer = {
  _fromJsonObject(object: any): S3Config {
    return {
      accessKeyId: object["accessKeyId"],
      bucket: object["bucket"],
      endpointUrl: object["endpointUrl"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: S3Config): any {
    return {
      accessKeyId: self.accessKeyId,
      bucket: self.bucket,
      endpointUrl: self.endpointUrl,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
