// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessageSubscriberAuthTokenOut {
  bridgeToken: string;
  token: string;
}

export const MessageSubscriberAuthTokenOutSerializer = {
  _fromJsonObject(object: any): MessageSubscriberAuthTokenOut {
    return {
      bridgeToken: object["bridgeToken"],
      token: object["token"],
    };
  },

  _toJsonObject(self: MessageSubscriberAuthTokenOut): any {
    return {
      bridgeToken: self.bridgeToken,
      token: self.token,
    };
  },
};
