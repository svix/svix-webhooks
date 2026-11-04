// this file is @generated

export interface DestinationTransformIn {
  code?: string | null;
}

export const DestinationTransformInSerializer = {
  _fromJsonObject(object: any): DestinationTransformIn {
    return {
      code: object["code"],
    };
  },

  _toJsonObject(self: DestinationTransformIn): any {
    return {
      code: self.code,
    };
  },
};
