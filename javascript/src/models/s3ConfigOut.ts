// this file is @generated

export interface S3ConfigOut {
  bucket: string;
  accessKeyId: string;
  region: string;
  endpointUrl?: string | null;
}

export const S3ConfigOutSerializer = {
  _fromJsonObject(object: any): S3ConfigOut {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: S3ConfigOut): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
