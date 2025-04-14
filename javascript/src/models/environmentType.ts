// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum EnvironmentType {
  Development = "development",
  Production = "production",
}

export const EnvironmentTypeSerializer = {
  _fromJsonObject(object: any): EnvironmentType {
    return object;
  },

  _toJsonObject(self: EnvironmentType): any {
    return self;
  },
};
