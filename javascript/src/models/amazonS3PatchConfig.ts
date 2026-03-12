// this file is @generated

export interface AmazonS3PatchConfig {
  accessKeyId?: string;
  bucket?: string;
  region?: string;
  secretAccessKey?: string;
}

export const AmazonS3PatchConfigSerializer = {
  _fromJsonObject(object: any): AmazonS3PatchConfig {
    return {
      accessKeyId: object["accessKeyId"],
      bucket: object["bucket"],
      region: object["region"],
      secretAccessKey: object["secretAccessKey"],
    };
  },

  _toJsonObject(self: AmazonS3PatchConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      bucket: self.bucket,
      region: self.region,
      secretAccessKey: self.secretAccessKey,
    };
  },
};
