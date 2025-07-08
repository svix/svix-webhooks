// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AirwallexConfig {
  secret: string;
}

export const AirwallexConfigSerializer = {
  _fromJsonObject(object: any): AirwallexConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: AirwallexConfig): any {
    return {
      secret: self.secret,
    };
  },
};
