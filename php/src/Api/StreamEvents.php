<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\CreateStreamEventsIn;
use Svix\Models\CreateStreamEventsOut;
use Svix\Models\EventStreamOut;
use Svix\Request\SvixHttpClient;

class StreamEvents
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Creates events on the Stream. */
    public function create(
        string $streamId,
        CreateStreamEventsIn $createStreamEventsIn,
        ?StreamEventsCreateOptions $options = null,
    ): CreateStreamEventsOut {
        $request = $this->client->newReq('POST', "/api/v1/stream/{$streamId}/events");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($createStreamEventsIn));
        $res = $this->client->send($request);

        return CreateStreamEventsOut::fromJson($res);
    }

    /**
     * Iterate over a stream of events.
     *
     * The sink must be of type `poller` to use the poller endpoint.
     */
    public function get(
        string $streamId,
        string $sinkId,
        ?StreamEventsGetOptions $options = null,
    ): EventStreamOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}/events");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('after', $options->after);
        }
        $res = $this->client->send($request);

        return EventStreamOut::fromJson($res);
    }
}
