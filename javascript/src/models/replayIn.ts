// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ReplayIn {
  since: Date;
  until?: Date | null;
}

export const ReplayInSerializer = {
  _fromJsonObject(object: any): ReplayIn {
    return {
      since: new Date(object["since"]),
      until: object["until"] ? new Date(object["until"]) : null,
    };
  },

  _toJsonObject(self: ReplayIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  },
};
