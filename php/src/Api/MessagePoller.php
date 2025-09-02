<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\PollingEndpointConsumerSeekIn;
use Svix\Models\PollingEndpointConsumerSeekOut;
use Svix\Models\PollingEndpointOut;
use Svix\Request\SvixHttpClient;

class MessagePoller
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Reads the stream of created messages for an application, filtered on the Sink's event types and Channels. */
    public function poll(
        string $appId,
        string $sinkId,
        ?MessagePollerPollOptions $options = null,
    ): PollingEndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/poller/{$sinkId}");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('event_type', $options->eventType);
            $request->setQueryParam('channel', $options->channel);
            $request->setQueryParam('after', $options->after);
        }
        $res = $this->client->send($request);

        return PollingEndpointOut::fromJson($res);
    }

    /**
     * Reads the stream of created messages for an application, filtered on the Sink's event types and
     * Channels, using server-managed iterator tracking.
     */
    public function consumerPoll(
        string $appId,
        string $sinkId,
        string $consumerId,
        ?MessagePollerConsumerPollOptions $options = null,
    ): PollingEndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/poller/{$sinkId}/consumer/{$consumerId}");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
        }
        $res = $this->client->send($request);

        return PollingEndpointOut::fromJson($res);
    }

    /** Sets the starting offset for the consumer of a polling endpoint. */
    public function consumerSeek(
        string $appId,
        string $sinkId,
        string $consumerId,
        PollingEndpointConsumerSeekIn $pollingEndpointConsumerSeekIn,
        ?MessagePollerConsumerSeekOptions $options = null,
    ): PollingEndpointConsumerSeekOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/poller/{$sinkId}/consumer/{$consumerId}/seek");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($pollingEndpointConsumerSeekIn));
        $res = $this->client->send($request);

        return PollingEndpointConsumerSeekOut::fromJson($res);
    }
}
