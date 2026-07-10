// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type OperationalWebhook struct {
	client *internal.SvixHttpClient
}

func newOperationalWebhook(client *internal.SvixHttpClient) OperationalWebhook {
	return OperationalWebhook{client}
}

func (operationalWebhook OperationalWebhook) Endpoint() OperationalWebhookEndpoint {
	return newOperationalWebhookEndpoint(operationalWebhook.client)
}
