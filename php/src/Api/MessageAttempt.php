<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ListResponseEndpointMessageOut;
use Svix\Models\ListResponseMessageAttemptOut;
use Svix\Models\ListResponseMessageEndpointOut;
use Svix\Models\MessageAttemptOut;
use Svix\Request\SvixHttpClient;

class MessageAttempt
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * List attempts by endpoint id.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated
     * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public function listByEndpoint(
        string $appId,
        string $endpointId,
        ?MessageAttemptListByEndpointOptions $options = null,
    ): ListResponseMessageAttemptOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/attempt/endpoint/{$endpointId}");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('status', $options->status);
            $request->setQueryParam('status_code_class', $options->statusCodeClass);
            $request->setQueryParam('channel', $options->channel);
            $request->setQueryParam('tag', $options->tag);
            $request->setQueryParam('before', $options->before);
            $request->setQueryParam('after', $options->after);
            $request->setQueryParam('with_content', $options->withContent);
            $request->setQueryParam('with_msg', $options->withMsg);
            $request->setQueryParam('event_types', $options->eventTypes);
        }
        $res = $this->client->send($request);

        return ListResponseMessageAttemptOut::fromJson($res);
    }

    /**
     * List attempts by message ID.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated
     * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public function listByMsg(
        string $appId,
        string $msgId,
        ?MessageAttemptListByMsgOptions $options = null,
    ): ListResponseMessageAttemptOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/attempt/msg/{$msgId}");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('status', $options->status);
            $request->setQueryParam('status_code_class', $options->statusCodeClass);
            $request->setQueryParam('channel', $options->channel);
            $request->setQueryParam('tag', $options->tag);
            $request->setQueryParam('endpoint_id', $options->endpointId);
            $request->setQueryParam('before', $options->before);
            $request->setQueryParam('after', $options->after);
            $request->setQueryParam('with_content', $options->withContent);
            $request->setQueryParam('event_types', $options->eventTypes);
        }
        $res = $this->client->send($request);

        return ListResponseMessageAttemptOut::fromJson($res);
    }

    /**
     * List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
     *
     * The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated
     * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public function listAttemptedMessages(
        string $appId,
        string $endpointId,
        ?MessageAttemptListAttemptedMessagesOptions $options = null,
    ): ListResponseEndpointMessageOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/msg");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('channel', $options->channel);
            $request->setQueryParam('tag', $options->tag);
            $request->setQueryParam('status', $options->status);
            $request->setQueryParam('before', $options->before);
            $request->setQueryParam('after', $options->after);
            $request->setQueryParam('with_content', $options->withContent);
            $request->setQueryParam('event_types', $options->eventTypes);
        }
        $res = $this->client->send($request);

        return ListResponseEndpointMessageOut::fromJson($res);
    }

    /** `msg_id`: Use a message id or a message `eventId` */
    public function get(
        string $appId,
        string $msgId,
        string $attemptId,
    ): MessageAttemptOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/msg/{$msgId}/attempt/{$attemptId}");
        $res = $this->client->send($request);

        return MessageAttemptOut::fromJson($res);
    }

    /**
     * Deletes the given attempt's response body.
     *
     * Useful when an endpoint accidentally returned sensitive content.
     * The message can't be replayed or resent once its payload has been deleted or expired.
     */
    public function expungeContent(
        string $appId,
        string $msgId,
        string $attemptId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/msg/{$msgId}/attempt/{$attemptId}/content");
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * List endpoints attempted by a given message.
     *
     * Additionally includes metadata about the latest message attempt.
     * By default, endpoints are listed in ascending order by ID.
     */
    public function listAttemptedDestinations(
        string $appId,
        string $msgId,
        ?MessageAttemptListAttemptedDestinationsOptions $options = null,
    ): ListResponseMessageEndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/msg/{$msgId}/endpoint");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
        }
        $res = $this->client->send($request);

        return ListResponseMessageEndpointOut::fromJson($res);
    }

    /** Resend a message to the specified endpoint. */
    public function resend(
        string $appId,
        string $msgId,
        string $endpointId,
        ?MessageAttemptResendOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/msg/{$msgId}/endpoint/{$endpointId}/resend");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->sendNoResponseBody($request);
    }
}
