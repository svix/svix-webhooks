// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/models"
)

type MessagePoller struct {
	client *SvixHttpClient
}

func newMessagePoller(client *SvixHttpClient) *MessagePoller {
	return &MessagePoller{
		client: client,
	}
}

type MessagePollerPollOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	// Filters messages sent with this event type (optional).
	EventType *string
	// Filters messages sent with this channel (optional).
	Channel *string
	After   *time.Time
}

type MessagePollerConsumerPollOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	// Filters messages sent with this event type (optional).
	EventType *string
	// Filters messages sent with this channel (optional).
	Channel *string
}

type MessagePollerConsumerSeekOptions struct {
	IdempotencyKey *string
}

// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
func (messagePoller *MessagePoller) Poll(
	ctx context.Context,
	appId string,
	sinkId string,
	o *MessagePollerPollOptions,
) (*models.PollingEndpointOut, error) {
	pathMap := map[string]string{
		"app_id":  appId,
		"sink_id": sinkId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("event_type", o.EventType, queryMap, &err)
		serializeParamToMap("channel", o.Channel, queryMap, &err)
		serializeParamToMap("after", o.After, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return executeRequest[any, models.PollingEndpointOut](
		ctx,
		messagePoller.client,
		"GET",
		"/api/v1/app/{app_id}/poller/{sink_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Reads the stream of created messages for an application, filtered on the Sink's event types and
// Channels, using server-managed iterator tracking.
func (messagePoller *MessagePoller) ConsumerPoll(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	o *MessagePollerConsumerPollOptions,
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
		messagePoller.client,
		"GET",
		"/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Sets the starting offset for the consumer of a polling endpoint.
func (messagePoller *MessagePoller) ConsumerSeek(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	pollingEndpointConsumerSeekIn models.PollingEndpointConsumerSeekIn,
	o *MessagePollerConsumerSeekOptions,
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
		messagePoller.client,
		"POST",
		"/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
		pathMap,
		nil,
		headerMap,
		&pollingEndpointConsumerSeekIn,
	)
}
