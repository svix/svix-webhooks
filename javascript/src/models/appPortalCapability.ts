// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum AppPortalCapability {
  ViewBase = "ViewBase",
  ViewEndpointSecret = "ViewEndpointSecret",
  ManageEndpointSecret = "ManageEndpointSecret",
  ManageTransformations = "ManageTransformations",
  CreateAttempts = "CreateAttempts",
  ManageEndpoint = "ManageEndpoint",
}

export const AppPortalCapabilitySerializer = {
  _fromJsonObject(object: any): AppPortalCapability {
    return object;
  },

  _toJsonObject(self: AppPortalCapability): any {
    return self;
  },
};
