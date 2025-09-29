// this file is @generated

export interface DashboardAccessOut {
  token: string;
  url: string;
}

export const DashboardAccessOutSerializer = {
  _fromJsonObject(object: any): DashboardAccessOut {
    return {
      token: object["token"],
      url: object["url"],
    };
  },

  _toJsonObject(self: DashboardAccessOut): any {
    return {
      token: self.token,
      url: self.url,
    };
  },
};
