// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type EventsPublic struct {
	client *SvixHttpClient
}

func newEventsPublic(client *SvixHttpClient) *EventsPublic {
	return &EventsPublic{
		client: client,
	}
}

type EventsPublicConsumerOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	// Filters messages sent with this event type (optional).
	EventType *string
	// Filters messages sent with this channel (optional).
	Channel *string
}

type EventsPublicConsumerSeekOptions struct {
	IdempotencyKey *string
}

// Reads the stream of created messages for an application, filtered on the Sink's event types and
// Channels, using server-managed iterator tracking.
func (eventsPublic *EventsPublic) Consumer(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	o *EventsPublicConsumerOptions,
) (*models.PollingEndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"sink_id":     sinkId,
		"consumer_id": consumerId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("event_type", o.EventType, queryMap, &err)
		serializeParamToMap("channel", o.Channel, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return executeRequest[any, models.PollingEndpointOut](
		ctx,
		eventsPublic.client,
		"GET",
		"/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Sets the starting offset for the consumer of a polling endpoint.
func (eventsPublic *EventsPublic) ConsumerSeek(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	pollingEndpointConsumerSeekIn models.PollingEndpointConsumerSeekIn,
	o *EventsPublicConsumerSeekOptions,
) (*models.PollingEndpointConsumerSeekOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"sink_id":     sinkId,
		"consumer_id": consumerId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return executeRequest[models.PollingEndpointConsumerSeekIn, models.PollingEndpointConsumerSeekOut](
		ctx,
		eventsPublic.client,
		"POST",
		"/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
		pathMap,
		nil,
		headerMap,
		&pollingEndpointConsumerSeekIn,
	)
}
