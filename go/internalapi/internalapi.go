// Package internalapi, DO NOT USE THIS FILE, THE API WILL CHANGE WITHOUT WARNING!!!!
package internalapi

import (
	"fmt"
	"net/url"

	"github.com/svix/svix-webhooks/go/internal"
)

type (
	// THIS TYPE CAN AND WILL CHANGE WITHOUT WARNING
	InternalSvix struct {
		Management *Management
		Endpoint   *Endpoint
	}
)

func New(token string, serverUrl *url.URL, debug bool, userAgentSuffix *string) (*InternalSvix, error) {
	svixHttpClient := internal.DefaultSvixHttpClient(serverUrl.String())
	svixHttpClient.Debug = debug

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	userAgent := fmt.Sprintf("svix-libs/%s/go", internal.Version)
	if userAgentSuffix != nil {
		userAgent = fmt.Sprintf("%s/%s", userAgent, *userAgentSuffix)
	}
	svixHttpClient.DefaultHeaders["User-Agent"] = userAgent

	svx := InternalSvix{
		Management: newManagement(&svixHttpClient),
		Endpoint:   newEndpoint(&svixHttpClient),
	}
	return &svx, nil
}
