// this file is @generated
/**
 * How an endpoint must respond to a challenge.
 *
 * - `challenge-response`: the Standard Webhooks challenge-response check.
 *   The endpoint must return `webhook-challenge-response`, calculated from
 *   the challenge and its secret.
 * - `echo`: the endpoint must echo the challenge from the request body back
 *   as `{"challenge": "..."}`. This doesn't prove that it holds the secret.
 */
export enum EndpointChallengeMode {
  ChallengeResponse = "challenge-response",
  Echo = "echo",
}

export const EndpointChallengeModeSerializer = {
  _fromJsonObject(object: any): EndpointChallengeMode {
    return object;
  },

  _toJsonObject(self: EndpointChallengeMode): any {
    return self;
  },
};
