// this file is @generated

export interface EndpointStats {
  canceled: number;
  fail: number;
  pending: number;
  sending: number;
  success: number;
}

export const EndpointStatsSerializer = {
  _fromJsonObject(object: any): EndpointStats {
    return {
      canceled: object["canceled"],
      fail: object["fail"],
      pending: object["pending"],
      sending: object["sending"],
      success: object["success"],
    };
  },

  _toJsonObject(self: EndpointStats): any {
    return {
      canceled: self.canceled,
      fail: self.fail,
      pending: self.pending,
      sending: self.sending,
      success: self.success,
    };
  },
};
