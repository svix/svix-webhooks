// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface GithubConfig {
  secret?: string | null;
}

export const GithubConfigSerializer = {
  _fromJsonObject(object: any): GithubConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: GithubConfig): any {
    return {
      secret: self.secret,
    };
  },
};
