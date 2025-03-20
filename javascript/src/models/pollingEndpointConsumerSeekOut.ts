// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface PollingEndpointConsumerSeekOut {
  iterator: string;
}

export const PollingEndpointConsumerSeekOutSerializer = {
  _fromJsonObject(object: any): PollingEndpointConsumerSeekOut {
    return {
      iterator: object["iterator"],
    };
  },

  _toJsonObject(self: PollingEndpointConsumerSeekOut): any {
    return {
      iterator: self.iterator,
    };
  },
};
