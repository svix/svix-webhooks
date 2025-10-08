<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ListResponseStreamOut;
use Svix\Models\StreamIn;
use Svix\Models\StreamOut;
use Svix\Models\StreamPatch;
use Svix\Request\SvixHttpClient;

class StreamStream
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List of all the organization's streams. */
    public function list(
        ?StreamStreamListOptions $options = null,
    ): ListResponseStreamOut {
        $request = $this->client->newReq('GET', '/api/v1/stream');
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseStreamOut::fromJson($res);
    }

    /** Creates a new stream. */
    public function create(
        StreamIn $streamIn,
        ?StreamStreamCreateOptions $options = null,
    ): StreamOut {
        $request = $this->client->newReq('POST', '/api/v1/stream');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($streamIn));
        $res = $this->client->send($request);

        return StreamOut::fromJson($res);
    }

    /** Get a stream by id or uid. */
    public function get(
        string $streamId,
    ): StreamOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}");
        $res = $this->client->send($request);

        return StreamOut::fromJson($res);
    }

    /** Update a stream. */
    public function update(
        string $streamId,
        StreamIn $streamIn,
    ): StreamOut {
        $request = $this->client->newReq('PUT', "/api/v1/stream/{$streamId}");
        $request->setBody(json_encode($streamIn));
        $res = $this->client->send($request);

        return StreamOut::fromJson($res);
    }

    /** Delete a stream. */
    public function delete(
        string $streamId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/stream/{$streamId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update a stream. */
    public function patch(
        string $streamId,
        StreamPatch $streamPatch,
    ): StreamOut {
        $request = $this->client->newReq('PATCH', "/api/v1/stream/{$streamId}");
        $request->setBody(json_encode($streamPatch));
        $res = $this->client->send($request);

        return StreamOut::fromJson($res);
    }
}
