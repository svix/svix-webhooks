// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessagePrecheckOut {
  active: boolean;
}

export const MessagePrecheckOutSerializer = {
  _fromJsonObject(object: any): MessagePrecheckOut {
    return {
      active: object["active"],
    };
  },

  _toJsonObject(self: MessagePrecheckOut): any {
    return {
      active: self.active,
    };
  },
};
