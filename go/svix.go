// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"fmt"
	"net/http"
	"net/url"
	"regexp"
	"runtime"
	"strings"
	"time"

	"github.com/svix/svix-webhooks/go/internal"
)

type (
	SvixOptions struct {
		ServerUrl        *url.URL
		HTTPClient       *http.Client
		TransportWrapper func(http.RoundTripper) http.RoundTripper
		RetrySchedule    *[]time.Duration
		Debug            bool
	}
	Svix struct {
		// hidden field. allows me to override the user agent in `SvixHttpClient.DefaultHeaders["User-Agent"]`
		// to override the user agent use `SetUseragentSuffix`
		client *internal.SvixHttpClient

		Application                *Application
		Authentication             *Authentication
		BackgroundTask             *BackgroundTask
		Connector                  *Connector
		Endpoint                   *Endpoint
		Environment                *Environment
		EventType                  *EventType
		Health                     *Health
		Ingest                     *Ingest
		Integration                *Integration
		Message                    *Message
		MessageAttempt             *MessageAttempt
		OperationalWebhook         *OperationalWebhook
		Statistics                 *Statistics
		Streaming                  *Streaming
		OperationalWebhookEndpoint *OperationalWebhookEndpoint
	}
)

func New(token string, options *SvixOptions) (*Svix, error) {
	svixHttpClient := internal.DefaultSvixHttpClient(getDefaultBaseUrl(token))

	if options != nil {
		if options.ServerUrl != nil {
			svixHttpClient.BaseURL = options.ServerUrl.String()
		}
		if options.RetrySchedule != nil {
			if len(*options.RetrySchedule) > 5 {
				return nil, fmt.Errorf("number of retries must not exceed 5")
			}
			svixHttpClient.RetrySchedule = *options.RetrySchedule

		}
		if options.HTTPClient != nil {
			svixHttpClient.HTTPClient = options.HTTPClient
		}
		if options.TransportWrapper != nil {
			svixHttpClient.HTTPClient.Transport = options.TransportWrapper(svixHttpClient.HTTPClient.Transport)
		}
		svixHttpClient.Debug = options.Debug

	}

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	svixHttpClient.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go go/%s", Version, runtime.Version())

	svx := Svix{
		client: &svixHttpClient,

		Application:                newApplication(&svixHttpClient),
		Authentication:             newAuthentication(&svixHttpClient),
		BackgroundTask:             newBackgroundTask(&svixHttpClient),
		Connector:                  newConnector(&svixHttpClient),
		Endpoint:                   newEndpoint(&svixHttpClient),
		Environment:                newEnvironment(&svixHttpClient),
		EventType:                  newEventType(&svixHttpClient),
		Health:                     newHealth(&svixHttpClient),
		Ingest:                     newIngest(&svixHttpClient),
		Integration:                newIntegration(&svixHttpClient),
		Message:                    newMessage(&svixHttpClient),
		MessageAttempt:             newMessageAttempt(&svixHttpClient),
		OperationalWebhook:         newOperationalWebhook(&svixHttpClient),
		Statistics:                 newStatistics(&svixHttpClient),
		Streaming:                  newStreaming(&svixHttpClient),
		OperationalWebhookEndpoint: newOperationalWebhookEndpoint(&svixHttpClient),
	}
	return &svx, nil
}

// Add a custom suffix to the default user-agent
//
// The default user agent is `svix-libs/<version>/go go/<goversion>`.
// The suffix will be separated from the base svix-libs component of the user agent with a `/`
//
// The suffix must be less then 50 chars, And must match this regex `^[A-Za-z\d\.\-]+$`
//
// Deprecated: Please call the method with the same name instead of this free function.
func SetUserAgentSuffix(s *Svix, userAgentSuffix string) error {
	return s.SetUserAgentSuffix(userAgentSuffix)
}

// Add a custom suffix to the default user-agent
//
// The default user agent is `svix-libs/<version>/go go/<goversion>`.
// The suffix will be separated from the base svix-libs component of the user agent with a `/`
//
// The suffix must be less then 50 chars, And must match this regex `^[A-Za-z\d\.\-]+$`
func (s Svix) SetUserAgentSuffix(userAgentSuffix string) error {
	if len(userAgentSuffix) > 50 {
		return fmt.Errorf("user agent suffix must be less then 50 chars")
	}
	validateStr := regexp.MustCompile(`^[A-Za-z\d\.\-]+$`).MatchString
	if !validateStr(userAgentSuffix) {
		return fmt.Errorf("invalid user agent suffix")
	}

	s.client.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go/%s go/%s", Version, userAgentSuffix, runtime.Version())
	return nil
}

func getDefaultBaseUrl(token string) string {
	var tokenParts = strings.Split(token, ".")
	var region = tokenParts[len(tokenParts)-1]
	switch region {
	case "us":
		return "https://api.us.svix.com"
	case "eu":
		return "https://api.eu.svix.com"
	case "in":
		return "https://api.in.svix.com"
	case "ca":
		return "https://api.ca.svix.com"
	case "au":
		return "https://api.au.svix.com"
	default:
		return "https://api.svix.com"
	}
}
