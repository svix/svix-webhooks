// Package internalapi, DO NOT USE THIS FILE, THE API WILL CHANGE WITHOUT WARNING!!!!
package internalapi

import (
	"fmt"
	"net/url"

	svix "github.com/svix/svix-webhooks/go"
	"github.com/svix/svix-webhooks/go/internal"
)

type (
	// THIS TYPE CAN AND WILL CHANGE WITHOUT WARNING
	InternalSvix struct {
		Management *Management
	}
)

func New(token string, serverUrl *url.URL, debug bool, userAgentSuffix string) (*InternalSvix, error) {
	svixHttpClient := internal.DefaultSvixHttpClient(serverUrl.String())
	svixHttpClient.Debug = debug

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	svixHttpClient.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go/%s", svix.Version, userAgentSuffix)

	svx := InternalSvix{
		Management: newManagement(&svixHttpClient),
	}
	return &svx, nil
}
