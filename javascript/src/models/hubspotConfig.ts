// this file is @generated

export interface HubspotConfig {
  secret?: string | null;
}

export const HubspotConfigSerializer = {
  _fromJsonObject(object: any): HubspotConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: HubspotConfig): any {
    return {
      secret: self.secret,
    };
  },
};
