package svix

import (
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"time"
)

type (
	SvixOptions struct {
		ServerUrl     *url.URL
		HTTPClient    *http.Client
		RetrySchedule *[]time.Duration
		Debug         bool
	}
	Svix struct {
		Authentication             *Authentication
		Application                *Application
		Endpoint                   *Endpoint
		EventType                  *EventType
		Integration                *Integration
		Message                    *Message
		MessageAttempt             *MessageAttempt
		Statistics                 *Statistics
		OperationalWebhookEndpoint *OperationalWebhookEndpoint
	}
)

func New(token string, options *SvixOptions) (*Svix, error) {
	svixHttpClient := defaultSvixHttpClient()
	if options != nil && options.ServerUrl == nil {
		var tokenParts = strings.Split(token, ".")
		var region = tokenParts[len(tokenParts)-1]
		if region == "us" {
			svixHttpClient.BaseURL = "https://api.us.svix.com"
		} else if region == "eu" {
			svixHttpClient.BaseURL = "https://api.eu.svix.com"
		} else if region == "in" {
			svixHttpClient.BaseURL = "https://api.in.svix.com"
		}
	} else {
		svixHttpClient.BaseURL = options.ServerUrl.String()
	}

	if options != nil {
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
		Authentication: &Authentication{
			_client: &svixHttpClient,
		},
		Application: &Application{
			_client: &svixHttpClient,
		},
		Endpoint: &Endpoint{
			_client: &svixHttpClient,
		},
		EventType: &EventType{
			_client: &svixHttpClient,
		},
		Message: &Message{
			_client: &svixHttpClient,
		},
		Integration: &Integration{
			_client: &svixHttpClient,
		},
		MessageAttempt: &MessageAttempt{
			_client: &svixHttpClient,
		},
		Statistics: &Statistics{
			_client: &svixHttpClient,
		},
		OperationalWebhookEndpoint: &OperationalWebhookEndpoint{
			_client: &svixHttpClient,
		},
	}
	return &svx, nil
}
