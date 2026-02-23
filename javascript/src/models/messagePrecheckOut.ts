// this file is @generated

export interface MessagePrecheckOut {
  /** Whether there are any active endpoint that would get sent such a message. */
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
