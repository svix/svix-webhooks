// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type OperationalWebhook struct {
	Endpoint *OperationalWebhookEndpoint
}

func newOperationalWebhook(client *internal.SvixHttpClient) *OperationalWebhook {
	return &OperationalWebhook{
		Endpoint: newOperationalWebhookEndpoint(client),
	}
}
