// this file is @generated

export interface AppPortalAccessOut {
  token: string;
  url: string;
}

export const AppPortalAccessOutSerializer = {
  _fromJsonObject(object: any): AppPortalAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  },

  _toJsonObject(self: AppPortalAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  },
};
