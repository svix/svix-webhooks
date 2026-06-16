// this file is @generated

export interface PollerV2CommitIn {
  offset: number;
}

export const PollerV2CommitInSerializer = {
  _fromJsonObject(object: any): PollerV2CommitIn {
    return {
      offset: object["offset"],
    };
  },

  _toJsonObject(self: PollerV2CommitIn): any {
    return {
      offset: self.offset,
    };
  },
};
