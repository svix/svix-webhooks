// Package internalapi, DO NOT USE THIS FILE, THE API WILL CHANGE WITHOUT WARNING!!!!
package internalapi

import (
	"fmt"
	"net/url"
	"runtime"

	"github.com/svix/svix-webhooks/go/internal"
)

type (
	// THIS TYPE CAN AND WILL CHANGE WITHOUT WARNING
	InternalSvix struct {
		client *internal.SvixHttpClient
	}
)

func (svix InternalSvix) Endpoint() Endpoint {
	return newEndpoint(svix.client)
}

func (svix InternalSvix) Management() Management {
	return newManagement(svix.client)
}

func (svix InternalSvix) Message() Message {
	return newMessage(svix.client)
}

func New(token string, serverUrl *url.URL, debug bool, userAgentSuffix *string) (*InternalSvix, error) {
	svixHttpClient := internal.DefaultSvixHttpClient(serverUrl.String())
	svixHttpClient.Debug = debug

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	userAgent := fmt.Sprintf("svix-libs/%s/go go/%s", internal.Version, runtime.Version())
	if userAgentSuffix != nil {
		userAgent = fmt.Sprintf("%s/%s", userAgent, *userAgentSuffix)
	}
	svixHttpClient.DefaultHeaders["User-Agent"] = userAgent

	svx := InternalSvix{
		client: &svixHttpClient,
	}
	return &svx, nil
}
