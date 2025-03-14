// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessageRawPayloadOut {
  payload: string;
}

export const MessageRawPayloadOutSerializer = {
  _fromJsonObject(object: any): MessageRawPayloadOut {
    return {
      payload: object["payload"],
    };
  },

  _toJsonObject(self: MessageRawPayloadOut): any {
    return {
      payload: self.payload,
    };
  },
};
