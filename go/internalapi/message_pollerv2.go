// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type MessagePollerv2 struct {
	client *internal.SvixHttpClient
}

func newMessagePollerv2(client *internal.SvixHttpClient) *MessagePollerv2 {
	return &MessagePollerv2{
		client: client,
	}
}

type MessagePollerv2ConsumerPollOptions struct {
	Limit *uint64
	// Lease duration in milliseconds.
	LeaseDurationMs *uint64

	StartingPosition *models.StartingPosition
}

type MessagePollerv2ConsumerCommitOptions struct {
	IdempotencyKey *string
}

// Poll messages from a sink.
func (messagePollerv2 *MessagePollerv2) ConsumerPoll(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	o *MessagePollerv2ConsumerPollOptions,
) (*models.PollerV2PollOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"sink_id":     sinkId,
		"consumer_id": consumerId,
	}
	queryMap := map[string]string{}
	if o != nil {
		var err error

		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("lease_duration_ms", o.LeaseDurationMs, queryMap, &err)
		internal.SerializeParamToMap("starting_position", o.StartingPosition, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.PollerV2PollOut](
		ctx,
		messagePollerv2.client,
		"GET",
		"/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Ack a message offset for a sink's consumer.
func (messagePollerv2 *MessagePollerv2) ConsumerCommit(
	ctx context.Context,
	appId string,
	sinkId string,
	consumerId string,
	pollerV2CommitIn models.PollerV2CommitIn,
	o *MessagePollerv2ConsumerCommitOptions,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"sink_id":     sinkId,
		"consumer_id": consumerId,
	}
	headerMap := map[string]string{}
	if o != nil {
		var err error

		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err := internal.ExecuteRequest[models.PollerV2CommitIn, any](
		ctx,
		messagePollerv2.client,
		"POST",
		"/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
		pathMap,
		nil,
		headerMap,
		&pollerV2CommitIn,
	)
	return err
}
