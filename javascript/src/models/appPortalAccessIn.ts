// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  AppPortalCapability,
  AppPortalCapabilitySerializer,
} from "./appPortalCapability";
import { ApplicationIn, ApplicationInSerializer } from "./applicationIn";

export interface AppPortalAccessIn {
  /**
   * Optionally creates a new application while generating the access link.
   *
   * If the application id or uid that is used in the path already exists, this argument is ignored.
   */
  application?: ApplicationIn | null;
  /**
   * Custom capabilities attached to the token, You can combine as many capabilities as necessary.
   *
   * The `ViewBase` capability is always required
   *
   * - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint secret.
   *
   * - `ViewEndpointSecret`: Allows user to view the endpoint secret.
   *
   * - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.
   *
   * - `ManageTransformations`: Allows user to modify the endpoint transformations.
   *
   * - `CreateAttempts`: Allows user to replay missing messages and send example messages.
   *
   * - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint (including secrets)
   */
  capabilities?: AppPortalCapability[] | null;
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
  /** The set of feature flags the created token will have access to. */
  featureFlags?: string[];
  /**
   * Whether the app portal should be in read-only mode.
   *
   * @deprecated
   */
  readOnly?: boolean | null;
  /**
   * An optional session ID to attach to the token.
   *
   * When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
   */
  sessionId?: string | null;
}

export const AppPortalAccessInSerializer = {
  _fromJsonObject(object: any): AppPortalAccessIn {
    return {
      application: object["application"]
        ? ApplicationInSerializer._fromJsonObject(object["application"])
        : undefined,
      capabilities: object["capabilities"]?.map((item: AppPortalCapability) =>
        AppPortalCapabilitySerializer._fromJsonObject(item)
      ),
      expiry: object["expiry"],
      featureFlags: object["featureFlags"],
      readOnly: object["readOnly"],
      sessionId: object["sessionId"],
    };
  },

  _toJsonObject(self: AppPortalAccessIn): any {
    return {
      application: self.application
        ? ApplicationInSerializer._toJsonObject(self.application)
        : undefined,
      capabilities: self.capabilities?.map((item) =>
        AppPortalCapabilitySerializer._toJsonObject(item)
      ),
      expiry: self.expiry,
      featureFlags: self.featureFlags,
      readOnly: self.readOnly,
      sessionId: self.sessionId,
    };
  },
};
