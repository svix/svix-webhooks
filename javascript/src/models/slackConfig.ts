// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface SlackConfig {
  secret: string;
}

export const SlackConfigSerializer = {
  _fromJsonObject(object: any): SlackConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: SlackConfig): any {
    return {
      secret: self.secret,
    };
  },
};
