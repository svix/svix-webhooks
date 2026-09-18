// this file is @generated

export interface EndpointAttemptStats {
  success: number;
  fail: number;
  canceled: number;
}

export const EndpointAttemptStatsSerializer = {
  _fromJsonObject(object: any): EndpointAttemptStats {
    return {
      success: object["success"],
      fail: object["fail"],
      canceled: object["canceled"],
    };
  },

  _toJsonObject(self: EndpointAttemptStats): any {
    return {
      success: self.success,
      fail: self.fail,
      canceled: self.canceled,
    };
  },
};
