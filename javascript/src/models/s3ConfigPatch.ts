// this file is @generated

export interface S3ConfigPatch {
  bucket?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
  endpointUrl?: string;
}

export const S3ConfigPatchSerializer = {
  _fromJsonObject(object: any): S3ConfigPatch {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: S3ConfigPatch): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
