// this file is @generated

export interface DashboardAccessOut {
  url: string;
  token: string;
}

export const DashboardAccessOutSerializer = {
  _fromJsonObject(object: any): DashboardAccessOut {
    return {
      url: object["url"],
      token: object["token"],
    };
  },

  _toJsonObject(self: DashboardAccessOut): any {
    return {
      url: self.url,
      token: self.token,
    };
  },
};
