// this file is @generated

export interface HttpAttemptTimes {
  end: Date;
  start: Date;
}

export const HttpAttemptTimesSerializer = {
  _fromJsonObject(object: any): HttpAttemptTimes {
    return {
      end: new Date(object["end"]),
      start: new Date(object["start"]),
    };
  },

  _toJsonObject(self: HttpAttemptTimes): any {
    return {
      end: self.end,
      start: self.start,
    };
  },
};
