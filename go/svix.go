package svix

import (
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
	"github.com/svix/svix-webhooks/go/internal/version"
)

type (
	SvixOptions struct {
		ServerUrl     *url.URL
		HTTPClient    *http.Client
		RetrySchedule *[]time.Duration
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
	AggregateEventTypesOut                    = openapi.AggregateEventTypesOut
	ApplicationIn                             = openapi.ApplicationIn
	ApplicationOut                            = openapi.ApplicationOut
	ApplicationPatch                          = openapi.ApplicationPatch
	ApplicationTokenExpireIn                  = openapi.ApplicationTokenExpireIn
	AppPortalAccessIn                         = openapi.AppPortalAccessIn
	AppPortalAccessOut                        = openapi.AppPortalAccessOut
	AppUsageStatsIn                           = openapi.AppUsageStatsIn
	AppUsageStatsOut                          = openapi.AppUsageStatsOut
	BackgroundTaskOut                         = openapi.BackgroundTaskOut
	BackgroundTaskStatus                      = openapi.BackgroundTaskStatus
	BackgroundTaskType                        = openapi.BackgroundTaskType
	DashboardAccessOut                        = openapi.DashboardAccessOut
	EndpointHeadersIn                         = openapi.EndpointHeadersIn
	EndpointHeadersOut                        = openapi.EndpointHeadersOut
	EndpointHeadersPatchIn                    = openapi.EndpointHeadersPatchIn
	EndpointIn                                = openapi.EndpointIn
	EndpointMessageOut                        = openapi.EndpointMessageOut
	EndpointOut                               = openapi.EndpointOut
	EndpointPatch                             = openapi.EndpointPatch
	EndpointSecretOut                         = openapi.EndpointSecretOut
	EndpointSecretRotateIn                    = openapi.EndpointSecretRotateIn
	EndpointStats                             = openapi.EndpointStats
	EndpointTransformationIn                  = openapi.EndpointTransformationIn
	EndpointTransformationOut                 = openapi.EndpointTransformationOut
	EndpointUpdate                            = openapi.EndpointUpdate
	EventExampleIn                            = openapi.EventExampleIn
	EventTypeImportOpenApiIn                  = openapi.EventTypeImportOpenApiIn
	EventTypeImportOpenApiOut                 = openapi.EventTypeImportOpenApiOut
	EventTypeImportOpenApiOutData             = openapi.EventTypeImportOpenApiOutData
	EventTypeIn                               = openapi.EventTypeIn
	EventTypeOut                              = openapi.EventTypeOut
	EventTypePatch                            = openapi.EventTypePatch
	EventTypeUpdate                           = openapi.EventTypeUpdate
	IntegrationIn                             = openapi.IntegrationIn
	IntegrationKeyOut                         = openapi.IntegrationKeyOut
	IntegrationOut                            = openapi.IntegrationOut
	IntegrationUpdate                         = openapi.IntegrationUpdate
	ListResponseApplicationOut                = openapi.ListResponseApplicationOut
	ListResponseBackgroundTaskOut             = openapi.ListResponseBackgroundTaskOut
	ListResponseEndpointMessageOut            = openapi.ListResponseEndpointMessageOut
	ListResponseEndpointOut                   = openapi.ListResponseEndpointOut
	ListResponseEventTypeOut                  = openapi.ListResponseEventTypeOut
	ListResponseIntegrationOut                = openapi.ListResponseIntegrationOut
	ListResponseMessageAttemptOut             = openapi.ListResponseMessageAttemptOut
	ListResponseMessageEndpointOut            = openapi.ListResponseMessageEndpointOut
	ListResponseMessageOut                    = openapi.ListResponseMessageOut
	ListResponseOperationalWebhookEndpointOut = openapi.ListResponseOperationalWebhookEndpointOut
	MessageAttemptEndpointOut                 = openapi.MessageAttemptEndpointOut
	MessageAttemptOut                         = openapi.MessageAttemptOut
	MessageEndpointOut                        = openapi.MessageEndpointOut
	MessageIn                                 = openapi.MessageIn
	MessageOut                                = openapi.MessageOut
	MessageStatus                             = openapi.MessageStatus
	OperationalWebhookEndpointIn              = openapi.OperationalWebhookEndpointIn
	OperationalWebhookEndpointOut             = openapi.OperationalWebhookEndpointOut
	OperationalWebhookEndpointSecretIn        = openapi.OperationalWebhookEndpointSecretIn
	OperationalWebhookEndpointSecretOut       = openapi.OperationalWebhookEndpointSecretOut
	OperationalWebhookEndpointUpdate          = openapi.OperationalWebhookEndpointUpdate
	Ordering                                  = openapi.Ordering
	RecoverIn                                 = openapi.RecoverIn
	RecoverOut                                = openapi.RecoverOut
	ReplayIn                                  = openapi.ReplayIn
	ReplayOut                                 = openapi.ReplayOut
	StatusCodeClass                           = openapi.StatusCodeClass
	OperationalWebhookEndpointHeadersOut      = openapi.OperationalWebhookEndpointHeadersOut
	OperationalWebhookEndpointHeadersIn       = openapi.OperationalWebhookEndpointHeadersIn

	// Deprecated: Use EndpointGetStatsOptions
	EndpointStatsOptions = EndpointGetStatsOptions
	// Deprecated: Use ListResponseMessageAttemptOut
	ListResponseMessageAttemptEndpointOut = openapi.ListResponseMessageAttemptOut
)

func String(s string) *string {
	return &s
}

func NullableString(s *string) *openapi.NullableString {
	return openapi.NewNullableString(s)
}

func StaticNullableString(s string) openapi.NullableString {
	return *NullableString(String(s))
}

func NullableInt32(num *int32) *openapi.NullableInt32 {
	return openapi.NewNullableInt32(num)
}

func NullableInt64(num *int64) *openapi.NullableInt64 {
	return openapi.NewNullableInt64(num)
}

func Int32(i int32) *int32 {
	return &i
}

func NullableBool(b *bool) *openapi.NullableBool {
	return openapi.NewNullableBool(b)
}

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

	if options != nil && options.HTTPClient != nil {
		svixHttpClient.HTTPClient = options.HTTPClient
	}

	if options != nil && options.RetrySchedule != nil {
		if len(*options.RetrySchedule) > 5 {
			return nil, fmt.Errorf("number of retries must not exceed 5")
		}
		svixHttpClient.RetrySchedule = *options.RetrySchedule
	}

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	svixHttpClient.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go", version.Version)

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
