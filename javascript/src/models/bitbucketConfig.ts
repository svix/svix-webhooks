// this file is @generated

export interface BitbucketConfig {
  secret: string;
}

export const BitbucketConfigSerializer = {
  _fromJsonObject(object: any): BitbucketConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: BitbucketConfig): any {
    return {
      secret: self.secret,
    };
  },
};
