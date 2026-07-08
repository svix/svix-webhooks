<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\EmptyResponse;
use Svix\Models\SinkTransformationOut;
use Svix\Models\SinkTransformIn;
use Svix\Request\SvixHttpClient;

class StreamingSinkTransformation
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Get the transformation code associated with this sink.
     *
     * @throws ApiException
     */
    public function get(
        string $streamId,
        string $sinkId,
    ): SinkTransformationOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}/transformation");
        $res = $this->client->send($request);

        return SinkTransformationOut::fromJson($res);
    }

    /**
     * Set or unset the transformation code associated with this sink.
     *
     * @throws ApiException
     */
    public function patch(
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
