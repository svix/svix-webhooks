// this file is @generated

export interface AmazonS3PatchConfig {
  bucket?: string;
  accessKeyId?: string;
  secretAccessKey?: string;
  region?: string;
  endpointUrl?: string;
}

export const AmazonS3PatchConfigSerializer = {
  _fromJsonObject(object: any): AmazonS3PatchConfig {
    return {
      bucket: object["bucket"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: AmazonS3PatchConfig): any {
    return {
      bucket: self.bucket,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      region: self.region,
      endpointUrl: self.endpointUrl,
    };
  },
};
