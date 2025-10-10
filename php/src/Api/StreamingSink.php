<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\EmptyResponse;
use Svix\Models\EndpointSecretRotateIn;
use Svix\Models\ListResponseStreamSinkOut;
use Svix\Models\SinkSecretOut;
use Svix\Models\SinkTransformIn;
use Svix\Models\StreamSinkIn;
use Svix\Models\StreamSinkOut;
use Svix\Models\StreamSinkPatch;
use Svix\Request\SvixHttpClient;

class StreamingSink
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List of all the stream's sinks. */
    public function list(
        string $streamId,
        ?StreamingSinkListOptions $options = null,
    ): ListResponseStreamSinkOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseStreamSinkOut::fromJson($res);
    }

    /** Creates a new sink. */
    public function create(
        string $streamId,
        StreamSinkIn $streamSinkIn,
        ?StreamingSinkCreateOptions $options = null,
    ): StreamSinkOut {
        $request = $this->client->newReq('POST', "/api/v1/stream/{$streamId}/sink");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($streamSinkIn));
        $res = $this->client->send($request);

        return StreamSinkOut::fromJson($res);
    }

    /** Get a sink by id or uid. */
    public function get(
        string $streamId,
        string $sinkId,
    ): StreamSinkOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}");
        $res = $this->client->send($request);

        return StreamSinkOut::fromJson($res);
    }

    /** Update a sink. */
    public function update(
        string $streamId,
        string $sinkId,
        StreamSinkIn $streamSinkIn,
    ): StreamSinkOut {
        $request = $this->client->newReq('PUT', "/api/v1/stream/{$streamId}/sink/{$sinkId}");
        $request->setBody(json_encode($streamSinkIn));
        $res = $this->client->send($request);

        return StreamSinkOut::fromJson($res);
    }

    /** Delete a sink. */
    public function delete(
        string $streamId,
        string $sinkId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/stream/{$streamId}/sink/{$sinkId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update a sink. */
    public function patch(
        string $streamId,
        string $sinkId,
        StreamSinkPatch $streamSinkPatch,
    ): StreamSinkOut {
        $request = $this->client->newReq('PATCH', "/api/v1/stream/{$streamId}/sink/{$sinkId}");
        $request->setBody(json_encode($streamSinkPatch));
        $res = $this->client->send($request);

        return StreamSinkOut::fromJson($res);
    }

    /**
     * Get the sink's signing secret (only supported for http sinks).
     *
     * This is used to verify the authenticity of the delivery.
     *
     * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public function getSecret(
        string $streamId,
        string $sinkId,
    ): SinkSecretOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}/secret");
        $res = $this->client->send($request);

        return SinkSecretOut::fromJson($res);
    }

    /** Rotates the signing secret (only supported for http sinks). */
    public function rotateSecret(
        string $streamId,
        string $sinkId,
        EndpointSecretRotateIn $endpointSecretRotateIn,
        ?StreamingSinkRotateSecretOptions $options = null,
    ): EmptyResponse {
        $request = $this->client->newReq('POST', "/api/v1/stream/{$streamId}/sink/{$sinkId}/secret/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($endpointSecretRotateIn));
        $res = $this->client->send($request);

        return EmptyResponse::fromJson($res);
    }

    /** Set or unset the transformation code associated with this sink. */
    public function transformationPartialUpdate(
        string $streamId,
        string $sinkId,
        SinkTransformIn $sinkTransformIn,
    ): EmptyResponse {
        $request = $this->client->newReq('PATCH', "/api/v1/stream/{$streamId}/sink/{$sinkId}/transformation");
        $request->setBody(json_encode($sinkTransformIn));
        $res = $this->client->send($request);

        return EmptyResponse::fromJson($res);
    }
}
