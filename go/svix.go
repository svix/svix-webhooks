package svix

import (
	"fmt"
	"net/http"
	"net/url"
	"regexp"
	"strings"
	"time"

	"github.com/svix/svix-webhooks/go/internal"
)

type (
	SvixOptions struct {
		ServerUrl     *url.URL
		HTTPClient    *http.Client
		RetrySchedule *[]time.Duration
		Debug         bool
	}
	Svix struct {
		// hidden field. allows me to override the user agent in `SvixHttpClient.DefaultHeaders["User-Agent"]`
		// to override the user agent use `SetUseragentSuffix`
		client *internal.SvixHttpClient

		Authentication             *Authentication
		Application                *Application
		Endpoint                   *Endpoint
		Environment                *Environment
		EventType                  *EventType
		Ingest                     *Ingest
		Integration                *Integration
		Management                 *Management
		Message                    *Message
		MessageAttempt             *MessageAttempt
		Statistics                 *Statistics
		OperationalWebhook         *OperationalWebhook
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
		svixHttpClient.Debug = options.Debug

	}

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	svixHttpClient.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go", Version)

	svx := Svix{
		client: &svixHttpClient,

		Authentication:             newAuthentication(&svixHttpClient),
		Application:                newApplication(&svixHttpClient),
		Endpoint:                   newEndpoint(&svixHttpClient),
		Environment:                newEnvironment(&svixHttpClient),
		EventType:                  newEventType(&svixHttpClient),
		Message:                    newMessage(&svixHttpClient),
		Ingest:                     newIngest(&svixHttpClient),
		Integration:                newIntegration(&svixHttpClient),
		Management:                 newManagement(&svixHttpClient),
		MessageAttempt:             newMessageAttempt(&svixHttpClient),
		Statistics:                 newStatistics(&svixHttpClient),
		OperationalWebhook:         newOperationalWebhook(&svixHttpClient),
		OperationalWebhookEndpoint: newOperationalWebhookEndpoint(&svixHttpClient),
	}
	return &svx, nil
}

// Add a custom suffix to the default user-agent
//
// The default user agent is `svix-libs/<version>/go`.
// The suffix will be separated from the base user agent with a `/`
//
// The suffix must be less then 50 chars, And must match this regex `^[A-Za-z\d\.\-]+$`
func SetUserAgentSuffix(s *Svix, userAgentSuffix string) error {
	if len(userAgentSuffix) > 50 {
		return fmt.Errorf("user agent suffix must be less then 50 chars")
	}
	validateStr := regexp.MustCompile(`^[A-Za-z\d\.\-]+$`).MatchString
	if !validateStr(userAgentSuffix) {
		return fmt.Errorf("invalid user agent suffix")
	}

	s.client.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go/%s", Version, userAgentSuffix)
	return nil
}

func getDefaultBaseUrl(token string) string {
	var tokenParts = strings.Split(token, ".")
	var region = tokenParts[len(tokenParts)-1]
	if region == "us" {
		return "https://api.us.svix.com"
	} else if region == "eu" {
		return "https://api.eu.svix.com"
	} else if region == "in" {
		return "https://api.in.svix.com"
	} else if region == "ca" {
		return "https://api.ca.svix.com"
	} else if region == "au" {
		return "https://api.au.svix.com"
	} else {
		return "https://api.svix.com"
	}
}
