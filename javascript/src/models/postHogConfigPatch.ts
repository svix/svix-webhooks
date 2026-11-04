// this file is @generated

export interface PostHogConfigPatch {
  domain?: string;
  projectToken?: string;
}

export const PostHogConfigPatchSerializer = {
  _fromJsonObject(object: any): PostHogConfigPatch {
    return {
      domain: object["domain"],
      projectToken: object["projectToken"],
    };
  },

  _toJsonObject(self: PostHogConfigPatch): any {
    return {
      domain: self.domain,
      projectToken: self.projectToken,
    };
  },
};
