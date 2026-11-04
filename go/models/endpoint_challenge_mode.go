// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// How an endpoint must respond to a challenge.
//
//   - `challenge-response`: the Standard Webhooks challenge-response check.
//     The endpoint must return `webhook-challenge-response`, calculated from
//     the challenge and its secret.
//   - `echo`: the endpoint must echo the challenge from the request body back
//     as `{"challenge": "..."}`. This doesn't prove that it holds the secret.
type EndpointChallengeMode string

const (
	ENDPOINTCHALLENGEMODE_CHALLENGE_RESPONSE EndpointChallengeMode = "challenge-response"
	ENDPOINTCHALLENGEMODE_ECHO               EndpointChallengeMode = "echo"
)

var allowedEndpointChallengeMode = []EndpointChallengeMode{
	"challenge-response",
	"echo",
}

func (v *EndpointChallengeMode) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := EndpointChallengeMode(value)
	if slices.Contains(allowedEndpointChallengeMode, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid EndpointChallengeMode", value)

}

var EndpointChallengeModeFromString = map[string]EndpointChallengeMode{
	"challenge-response": ENDPOINTCHALLENGEMODE_CHALLENGE_RESPONSE,
	"echo":               ENDPOINTCHALLENGEMODE_ECHO,
}
