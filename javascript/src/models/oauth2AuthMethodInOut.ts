// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/**
 * The method used for authenticating to the OAuth authorization server.
 *
 * `clientSecretJwt` will construct a JWT used for authentication with the oauth authorization server. This method is less commonly used and may not be supported by all oauth providers. `clientSecretBasic` will authenticate to the oauth authorization server using an `Authorization` header with the client secret as the value. This is the most common means of authentication. `clientSecretPost` will authenticate to the oauth authorization server by passing the client secret in a `client_secret` field in the request body. This method may not be supported by all oauth providers, and in general `clientSecretBasic` should be preferred.
 */
export enum Oauth2AuthMethodInOut {
  ClientSecretJwt = "clientSecretJwt",
  ClientSecretBasic = "clientSecretBasic",
  ClientSecretPost = "clientSecretPost",
}

export const Oauth2AuthMethodInOutSerializer = {
  _fromJsonObject(object: any): Oauth2AuthMethodInOut {
    return object;
  },

  _toJsonObject(self: Oauth2AuthMethodInOut): any {
    return self;
  },
};
