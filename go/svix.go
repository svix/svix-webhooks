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
		Debug bool

		// Overrides the base URL (protocol + hostname) used for all requests sent by this Svix client. (Useful for testing)
		ServerUrl  *url.URL
		HTTPClient *http.Client
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
	ListResponseMessageAttemptEndpointOut     = openapi.ListResponseMessageAttemptEndpointOut
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
)

var defaultHTTPClient = &http.Client{
	Timeout: 60 * time.Second,
}

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

func New(token string, options *SvixOptions) *Svix {
	conf := openapi.NewConfiguration()
	conf.Scheme = "https"
	conf.Host = "api.svix.com"
	conf.HTTPClient = defaultHTTPClient

	var tokenParts = strings.Split(token, ".")
	var region = tokenParts[len(tokenParts)-1]
	if region == "us" {
		conf.Host = "api.us.svix.com"
	} else if region == "eu" {
		conf.Host = "api.eu.svix.com"
	} else if region == "in" {
		conf.Host = "api.in.svix.com"
	}

	if options != nil {
		conf.Debug = options.Debug
		if options.ServerUrl != nil {
			conf.Scheme = options.ServerUrl.Scheme
			conf.Host = options.ServerUrl.Host
		}
		if options.HTTPClient != nil {
			conf.HTTPClient = options.HTTPClient
		}
	}
	conf.AddDefaultHeader("Authorization", fmt.Sprintf("Bearer %s", token))
	conf.UserAgent = fmt.Sprintf("svix-libs/%s/go", version.Version)
	apiClient := openapi.NewAPIClient(conf)
	return &Svix{
		Authentication: &Authentication{
			api: apiClient,
		},
		Application: &Application{
			api: apiClient,
		},
		Endpoint: &Endpoint{
			api: apiClient,
		},
		EventType: &EventType{
			api: apiClient,
		},
		Message: &Message{
			api: apiClient,
		},
		Integration: &Integration{
			api: apiClient,
		},
		MessageAttempt: &MessageAttempt{
			api: apiClient,
		},
		Statistics: &Statistics{
			api: apiClient,
		},
		OperationalWebhookEndpoint: &OperationalWebhookEndpoint{
			api: apiClient,
		},
	}
}
