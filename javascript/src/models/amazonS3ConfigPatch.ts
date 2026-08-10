// this file is @generated

export interface AmazonS3ConfigPatch {
  bucket?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
  endpointUrl?: string;
}

export const AmazonS3ConfigPatchSerializer = {
  _fromJsonObject(object: any): AmazonS3ConfigPatch {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: AmazonS3ConfigPatch): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
