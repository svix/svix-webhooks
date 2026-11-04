<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\DestinationTransformationOut;
use Svix\Models\DestinationTransformIn;
use Svix\Models\EmptyResponse;
use Svix\Request\SvixHttpClient;

class DestinationTransformation
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Get the transformation code associated with this destination.
     *
     * @throws ApiException
     */
    public function get(
        string $appId,
        string $destinationId,
    ): DestinationTransformationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/destination/{$destinationId}/transformation");
        $res = $this->client->send($request);

        return DestinationTransformationOut::fromJson($res);
    }

    /**
     * Set or unset the transformation code associated with this destination.
     *
     * @throws ApiException
     */
    public function patch(
        string $appId,
        string $destinationId,
        DestinationTransformIn $destinationTransformIn,
    ): EmptyResponse {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/destination/{$destinationId}/transformation");
        $request->setBody(json_encode($destinationTransformIn));
        $res = $this->client->send($request);

        return EmptyResponse::fromJson($res);
    }
}
