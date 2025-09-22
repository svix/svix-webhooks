// this file is @generated

export interface RecoverIn {
  since: Date;
  until?: Date | null;
}

export const RecoverInSerializer = {
  _fromJsonObject(object: any): RecoverIn {
    return {
      since: new Date(object["since"]),
      until: object["until"] ? new Date(object["until"]) : null,
    };
  },

  _toJsonObject(self: RecoverIn): any {
    return {
      since: self.since,
      until: self.until,
    };
  },
};
