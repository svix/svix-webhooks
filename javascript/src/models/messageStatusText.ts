// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum MessageStatusText {
  Success = "success",
  Pending = "pending",
  Fail = "fail",
  Sending = "sending",
}

export const MessageStatusTextSerializer = {
  _fromJsonObject(object: any): MessageStatusText {
    return object;
  },

  _toJsonObject(self: MessageStatusText): any {
    return self;
  },
};
