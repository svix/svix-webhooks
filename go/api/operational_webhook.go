// Package svix this file is @generated DO NOT EDIT
package api

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type OperationalWebhook struct {
	client *internal.SvixHttpClient
}

func NewOperationalWebhook(client *internal.SvixHttpClient) OperationalWebhook {
	return OperationalWebhook{client}
}

func (operationalWebhook OperationalWebhook) Endpoint() OperationalWebhookEndpoint {
	return NewOperationalWebhookEndpoint(operationalWebhook.client)
}
