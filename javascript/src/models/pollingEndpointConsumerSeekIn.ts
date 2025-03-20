// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface PollingEndpointConsumerSeekIn {
  after: Date;
}

export const PollingEndpointConsumerSeekInSerializer = {
  _fromJsonObject(object: any): PollingEndpointConsumerSeekIn {
    return {
      after: new Date(object["after"]),
    };
  },

  _toJsonObject(self: PollingEndpointConsumerSeekIn): any {
    return {
      after: self.after,
    };
  },
};
