// this file is @generated

export interface SinkTransformIn {
  code?: string | null;
}

export const SinkTransformInSerializer = {
  _fromJsonObject(object: any): SinkTransformIn {
    return {
      code: object["code"],
    };
  },

  _toJsonObject(self: SinkTransformIn): any {
    return {
      code: self.code,
    };
  },
};
