# this file is @generated
from enum import Enum


class EndpointChallengeMode(str, Enum):
    """How an endpoint must respond to a challenge.

    - `challenge-response`: the Standard Webhooks challenge-response check.
      The endpoint must return `webhook-challenge-response`, calculated from
      the challenge and its secret.
    - `echo`: the endpoint must echo the challenge from the request body back
      as `{"challenge": "..."}`. This doesn't prove that it holds the secret."""

    CHALLENGE_RESPONSE = "challenge-response"
    ECHO = "echo"

    def __str__(self) -> str:
        return str(self.value)
