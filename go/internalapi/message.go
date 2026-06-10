// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Message struct {
	Pollerv2 *MessagePollerv2
}

func newMessage(client *internal.SvixHttpClient) *Message {
	return &Message{
		Pollerv2: newMessagePollerv2(client),
	}
}
