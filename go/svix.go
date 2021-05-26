package svix

import (
	"fmt"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type (
	SvixOptions struct {
		Debug bool
	}
	Svix struct {
		Authentication *Authentication
		Application    *Application
		Endpoint       *Endpoint
		EventType      *EventType
		Message        *Message
		MessageAttempt *MessageAttempt
	}
)

func New(token string, options *SvixOptions) *Svix {
	conf := openapi.NewConfiguration()
	if options != nil {
		conf.Debug = options.Debug
	}
	conf.AddDefaultHeader("Authorization", fmt.Sprintf("Bearer %s", token))
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
		MessageAttempt: &MessageAttempt{
			api: apiClient,
		},
	}
}
