// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/v2/go/internal"
)

type Message struct {
	client *internal.SvixHttpClient
}

func newMessage(client *internal.SvixHttpClient) Message {
	return Message{client}
}

func (message Message) Pollerv2() MessagePollerv2 {
	return newMessagePollerv2(message.client)
}
