// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CheckbookConfig {
  secret: string;
}

export const CheckbookConfigSerializer = {
  _fromJsonObject(object: any): CheckbookConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: CheckbookConfig): any {
    return {
      secret: self.secret,
    };
  },
};
