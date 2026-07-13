// this file is @generated

export interface EndpointStats {
  success: number;
  pending: number;
  sending: number;
  fail: number;
  canceled: number;
}

export const EndpointStatsSerializer = {
  _fromJsonObject(object: any): EndpointStats {
    return {
      success: object["success"],
      pending: object["pending"],
      sending: object["sending"],
      fail: object["fail"],
      canceled: object["canceled"],
    };
  },

  _toJsonObject(self: EndpointStats): any {
    return {
      success: self.success,
      pending: self.pending,
      sending: self.sending,
      fail: self.fail,
      canceled: self.canceled,
    };
  },
};
