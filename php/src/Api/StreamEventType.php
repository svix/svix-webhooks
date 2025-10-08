<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ListResponseStreamEventTypeOut;
use Svix\Models\StreamEventTypeIn;
use Svix\Models\StreamEventTypeOut;
use Svix\Models\StreamEventTypePatch;
use Svix\Request\SvixHttpClient;

class StreamEventType
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List of all the organization's event types for streaming. */
    public function list(
        ?StreamEventTypeListOptions $options = null,
    ): ListResponseStreamEventTypeOut {
        $request = $this->client->newReq('GET', '/api/v1/stream/event-type');
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
            $request->setQueryParam('include_archived', $options->includeArchived);
        }
        $res = $this->client->send($request);

        return ListResponseStreamEventTypeOut::fromJson($res);
    }

    /** Create an event type for Streams. */
    public function create(
        StreamEventTypeIn $streamEventTypeIn,
        ?StreamEventTypeCreateOptions $options = null,
    ): StreamEventTypeOut {
        $request = $this->client->newReq('POST', '/api/v1/stream/event-type');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($streamEventTypeIn));
        $res = $this->client->send($request);

        return StreamEventTypeOut::fromJson($res);
    }

    /** Get an event type. */
    public function get(
        string $name,
    ): StreamEventTypeOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/event-type/{$name}");
        $res = $this->client->send($request);

        return StreamEventTypeOut::fromJson($res);
    }

    /** Update or create a event type for Streams. */
    public function update(
        string $name,
        StreamEventTypeIn $streamEventTypeIn,
    ): StreamEventTypeOut {
        $request = $this->client->newReq('PUT', "/api/v1/stream/event-type/{$name}");
        $request->setBody(json_encode($streamEventTypeIn));
        $res = $this->client->send($request);

        return StreamEventTypeOut::fromJson($res);
    }

    /** Delete an event type. */
    public function delete(
        string $name,
        ?StreamEventTypeDeleteOptions $options = null,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/stream/event-type/{$name}");
        if (null !== $options) {
            $request->setQueryParam('expunge', $options->expunge);
        }
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Patch an event type for Streams. */
    public function patch(
        string $name,
        StreamEventTypePatch $streamEventTypePatch,
    ): StreamEventTypeOut {
        $request = $this->client->newReq('PATCH', "/api/v1/stream/event-type/{$name}");
        $request->setBody(json_encode($streamEventTypePatch));
        $res = $this->client->send($request);

        return StreamEventTypeOut::fromJson($res);
    }
}
