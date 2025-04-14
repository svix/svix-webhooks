// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type MessageAttempt struct {
	client *internal.SvixHttpClient
}

func newMessageAttempt(client *internal.SvixHttpClient) *MessageAttempt {
	return &MessageAttempt{
		client: client,
	}
}

type MessageAttemptListByEndpointOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
	Status *models.MessageStatus

	// Filter response based on the HTTP status code
	StatusCodeClass *models.StatusCodeClass
	// Filter response based on the channel
	Channel *string
	// Filter response based on the tag
	Tag *string
	// Only include items created before a certain date
	Before *time.Time
	// Only include items created after a certain date
	After *time.Time
	// When `true` attempt content is included in the response
	WithContent *bool
	// When `true`, the message information is included in the response
	WithMsg *bool
	// Filter response based on the event type
	EventTypes *[]string
}

type MessageAttemptListByMsgOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
	Status *models.MessageStatus

	// Filter response based on the HTTP status code
	StatusCodeClass *models.StatusCodeClass
	// Filter response based on the channel
	Channel *string
	// Filter response based on the tag
	Tag *string
	// Filter the attempts based on the attempted endpoint
	EndpointId *string
	// Only include items created before a certain date
	Before *time.Time
	// Only include items created after a certain date
	After *time.Time
	// When `true` attempt content is included in the response
	WithContent *bool
	// Filter response based on the event type
	EventTypes *[]string
}

type MessageAttemptListAttemptedMessagesOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	// Filter response based on the channel
	Channel *string
	// Filter response based on the message tags
	Tag *string

	// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
	Status *models.MessageStatus
	// Only include items created before a certain date
	Before *time.Time
	// Only include items created after a certain date
	After *time.Time
	// When `true` message payloads are included in the response
	WithContent *bool
	// Filter response based on the event type
	EventTypes *[]string
}

type MessageAttemptListAttemptedDestinationsOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
}

type MessageAttemptResendOptions struct {
	IdempotencyKey *string
}

// List attempts by endpoint id
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListByEndpoint(
	ctx context.Context,
	appId string,
	endpointId string,
	o *MessageAttemptListByEndpointOptions,
) (*models.ListResponseMessageAttemptOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("status", o.Status, queryMap, &err)
		internal.SerializeParamToMap("status_code_class", o.StatusCodeClass, queryMap, &err)
		internal.SerializeParamToMap("channel", o.Channel, queryMap, &err)
		internal.SerializeParamToMap("tag", o.Tag, queryMap, &err)
		internal.SerializeParamToMap("before", o.Before, queryMap, &err)
		internal.SerializeParamToMap("after", o.After, queryMap, &err)
		internal.SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		internal.SerializeParamToMap("with_msg", o.WithMsg, queryMap, &err)
		internal.SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseMessageAttemptOut](
		ctx,
		messageAttempt.client,
		"GET",
		"/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// List attempts by message ID.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListByMsg(
	ctx context.Context,
	appId string,
	msgId string,
	o *MessageAttemptListByMsgOptions,
) (*models.ListResponseMessageAttemptOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
		"msg_id": msgId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("status", o.Status, queryMap, &err)
		internal.SerializeParamToMap("status_code_class", o.StatusCodeClass, queryMap, &err)
		internal.SerializeParamToMap("channel", o.Channel, queryMap, &err)
		internal.SerializeParamToMap("tag", o.Tag, queryMap, &err)
		internal.SerializeParamToMap("endpoint_id", o.EndpointId, queryMap, &err)
		internal.SerializeParamToMap("before", o.Before, queryMap, &err)
		internal.SerializeParamToMap("after", o.After, queryMap, &err)
		internal.SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		internal.SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseMessageAttemptOut](
		ctx,
		messageAttempt.client,
		"GET",
		"/api/v1/app/{app_id}/attempt/msg/{msg_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
//
// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListAttemptedMessages(
	ctx context.Context,
	appId string,
	endpointId string,
	o *MessageAttemptListAttemptedMessagesOptions,
) (*models.ListResponseEndpointMessageOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("channel", o.Channel, queryMap, &err)
		internal.SerializeParamToMap("tag", o.Tag, queryMap, &err)
		internal.SerializeParamToMap("status", o.Status, queryMap, &err)
		internal.SerializeParamToMap("before", o.Before, queryMap, &err)
		internal.SerializeParamToMap("after", o.After, queryMap, &err)
		internal.SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		internal.SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseEndpointMessageOut](
		ctx,
		messageAttempt.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// `msg_id`: Use a message id or a message `eventId`
func (messageAttempt *MessageAttempt) Get(
	ctx context.Context,
	appId string,
	msgId string,
	attemptId string,
) (*models.MessageAttemptOut, error) {
	pathMap := map[string]string{
		"app_id":     appId,
		"msg_id":     msgId,
		"attempt_id": attemptId,
	}
	return internal.ExecuteRequest[any, models.MessageAttemptOut](
		ctx,
		messageAttempt.client,
		"GET",
		"/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Deletes the given attempt's response body.
//
// Useful when an endpoint accidentally returned sensitive content.
// The message can't be replayed or resent once its payload has been deleted or expired.
func (messageAttempt *MessageAttempt) ExpungeContent(
	ctx context.Context,
	appId string,
	msgId string,
	attemptId string,
) error {
	pathMap := map[string]string{
		"app_id":     appId,
		"msg_id":     msgId,
		"attempt_id": attemptId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		messageAttempt.client,
		"DELETE",
		"/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// List endpoints attempted by a given message.
//
// Additionally includes metadata about the latest message attempt.
// By default, endpoints are listed in ascending order by ID.
func (messageAttempt *MessageAttempt) ListAttemptedDestinations(
	ctx context.Context,
	appId string,
	msgId string,
	o *MessageAttemptListAttemptedDestinationsOptions,
) (*models.ListResponseMessageEndpointOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
		"msg_id": msgId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseMessageEndpointOut](
		ctx,
		messageAttempt.client,
		"GET",
		"/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
		pathMap,
		queryMap,
		nil,
		nil,
	)
}

// Resend a message to the specified endpoint.
func (messageAttempt *MessageAttempt) Resend(
	ctx context.Context,
	appId string,
	msgId string,
	endpointId string,
	o *MessageAttemptResendOptions,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"msg_id":      msgId,
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		messageAttempt.client,
		"POST",
		"/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
		pathMap,
		nil,
		headerMap,
		nil,
	)
	return err
}
