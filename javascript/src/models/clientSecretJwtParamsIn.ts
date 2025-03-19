// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  OauthJwsSigningAlgorithm,
  OauthJwsSigningAlgorithmSerializer,
} from "./oauthJwsSigningAlgorithm";

export interface ClientSecretJwtParamsIn {
  /** The base64-encoded secret used for signing the JWT. */
  secretBase64: string;
  /** Optional secret identifier. If supplied, this will be populated in the JWT header in the `kid` field. */
  secretId?: string | null;
  /** The algorithm used to sign the JWT. */
  signingAlgorithm: OauthJwsSigningAlgorithm;
  /** Optional number of seconds after which the JWT should expire. Defaults to 300 seconds. */
  tokenExpirySecs?: number | null;
}

export const ClientSecretJwtParamsInSerializer = {
  _fromJsonObject(object: any): ClientSecretJwtParamsIn {
    return {
      secretBase64: object["secretBase64"],
      secretId: object["secretId"],
      signingAlgorithm: OauthJwsSigningAlgorithmSerializer._fromJsonObject(
        object["signingAlgorithm"]
      ),
      tokenExpirySecs: object["tokenExpirySecs"],
    };
  },

  _toJsonObject(self: ClientSecretJwtParamsIn): any {
    return {
      secretBase64: self.secretBase64,
      secretId: self.secretId,
      signingAlgorithm: OauthJwsSigningAlgorithmSerializer._toJsonObject(
        self.signingAlgorithm
      ),
      tokenExpirySecs: self.tokenExpirySecs,
    };
  },
};
