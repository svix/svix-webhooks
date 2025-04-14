// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EnvironmentModelUpdate {
  name: string;
}

export const EnvironmentModelUpdateSerializer = {
  _fromJsonObject(object: any): EnvironmentModelUpdate {
    return {
      name: object["name"],
    };
  },

  _toJsonObject(self: EnvironmentModelUpdate): any {
    return {
      name: self.name,
    };
  },
};
