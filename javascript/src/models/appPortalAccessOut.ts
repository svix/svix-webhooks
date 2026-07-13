// this file is @generated

export interface AppPortalAccessOut {
  url: string;
  token: string;
}

export const AppPortalAccessOutSerializer = {
  _fromJsonObject(object: any): AppPortalAccessOut {
    return {
      url: object["url"],
      token: object["token"],
    };
  },

  _toJsonObject(self: AppPortalAccessOut): any {
    return {
      url: self.url,
      token: self.token,
    };
  },
};
