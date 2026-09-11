// this file is @generated
import {
  type EndpointSecretRotateIn,
  EndpointSecretRotateInSerializer,
} from "./endpointSecretRotateIn";

export interface RotateSubscriptionIn2 {
  signingSecret?: EndpointSecretRotateIn | null;
}

export const RotateSubscriptionIn2Serializer = {
  _fromJsonObject(object: any): RotateSubscriptionIn2 {
    return {
      signingSecret:
        object["signingSecret"] != null
          ? EndpointSecretRotateInSerializer._fromJsonObject(object["signingSecret"])
          : undefined,
    };
  },

  _toJsonObject(self: RotateSubscriptionIn2): any {
    return {
      signingSecret:
        self.signingSecret != null
          ? EndpointSecretRotateInSerializer._toJsonObject(self.signingSecret)
          : undefined,
    };
  },
};
