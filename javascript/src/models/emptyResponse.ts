// this file is @generated

// biome-ignore-all lint/suspicious/noEmptyInterface: backwards compat

export interface EmptyResponse {}

export const EmptyResponseSerializer = {
  _fromJsonObject(_object: any): EmptyResponse {
    return {};
  },

  _toJsonObject(_self: EmptyResponse): any {
    return {};
  },
};
