// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AdobeSignConfig {
  clientId: string;
}

export const AdobeSignConfigSerializer = {
  _fromJsonObject(object: any): AdobeSignConfig {
    return {
      clientId: object["clientId"],
    };
  },

  _toJsonObject(self: AdobeSignConfig): any {
    return {
      clientId: self.clientId,
    };
  },
};
