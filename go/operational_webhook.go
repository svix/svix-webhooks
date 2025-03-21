// Package svix this file is @generated DO NOT EDIT
package svix

type OperationalWebhook struct {
	Endpoint *OperationalWebhookEndpoint
}

func newOperationalWebhook(client *SvixHttpClient) *OperationalWebhook {
	return &OperationalWebhook{
		Endpoint: newOperationalWebhookEndpoint(client),
	}
}
