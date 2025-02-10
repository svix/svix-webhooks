// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/models"
)

type MessageAttempt struct {
	_client *SvixHttpClient
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
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("status", o.Status, queryMap, &err)
		SerializeParamToMap("status_code_class", o.StatusCodeClass, queryMap, &err)
		SerializeParamToMap("channel", o.Channel, queryMap, &err)
		SerializeParamToMap("tag", o.Tag, queryMap, &err)
		SerializeParamToMap("before", o.Before, queryMap, &err)
		SerializeParamToMap("after", o.After, queryMap, &err)
		SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		SerializeParamToMap("with_msg", o.WithMsg, queryMap, &err)
		SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseMessageAttemptOut](
		ctx,
		messageAttempt._client,
		"GET",
		"/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
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
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("status", o.Status, queryMap, &err)
		SerializeParamToMap("status_code_class", o.StatusCodeClass, queryMap, &err)
		SerializeParamToMap("channel", o.Channel, queryMap, &err)
		SerializeParamToMap("tag", o.Tag, queryMap, &err)
		SerializeParamToMap("endpoint_id", o.EndpointId, queryMap, &err)
		SerializeParamToMap("before", o.Before, queryMap, &err)
		SerializeParamToMap("after", o.After, queryMap, &err)
		SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseMessageAttemptOut](
		ctx,
		messageAttempt._client,
		"GET",
		"/api/v1/app/{app_id}/attempt/msg/{msg_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
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
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("channel", o.Channel, queryMap, &err)
		SerializeParamToMap("tag", o.Tag, queryMap, &err)
		SerializeParamToMap("status", o.Status, queryMap, &err)
		SerializeParamToMap("before", o.Before, queryMap, &err)
		SerializeParamToMap("after", o.After, queryMap, &err)
		SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		SerializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseEndpointMessageOut](
		ctx,
		messageAttempt._client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
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
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[models.MessageAttemptOut](
		ctx,
		messageAttempt._client,
		"GET",
		"/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
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
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	_, apiErr := executeRequest[any](
		ctx,
		messageAttempt._client,
		"DELETE",
		"/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return apiErr
	}
	return nil
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
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseMessageEndpointOut](
		ctx,
		messageAttempt._client,
		"GET",
		"/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
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
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, apiErr := executeRequest[any](
		ctx,
		messageAttempt._client,
		"POST",
		"/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return apiErr
	}
	return nil
}
