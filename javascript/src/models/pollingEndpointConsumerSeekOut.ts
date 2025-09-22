// this file is @generated

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
