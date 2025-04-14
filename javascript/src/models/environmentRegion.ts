// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum EnvironmentRegion {
  Eu = "eu",
  Us = "us",
  In = "in",
  Au = "au",
  Ca = "ca",
  SelfHosted = "self-hosted",
}

export const EnvironmentRegionSerializer = {
  _fromJsonObject(object: any): EnvironmentRegion {
    return object;
  },

  _toJsonObject(self: EnvironmentRegion): any {
    return self;
  },
};
