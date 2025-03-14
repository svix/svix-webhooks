// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface HubspotOauthConfigIn {
  refreshToken: string;
}

export const HubspotOauthConfigInSerializer = {
  _fromJsonObject(object: any): HubspotOauthConfigIn {
    return {
      refreshToken: object["refresh_token"],
    };
  },

  _toJsonObject(self: HubspotOauthConfigIn): any {
    return {
      refresh_token: self.refreshToken,
    };
  },
};
