// this file is @generated

export interface PostHogConfigOut {
  domain: string;
}

export const PostHogConfigOutSerializer = {
  _fromJsonObject(object: any): PostHogConfigOut {
    return {
      domain: object["domain"],
    };
  },

  _toJsonObject(self: PostHogConfigOut): any {
    return {
      domain: self.domain,
    };
  },
};
