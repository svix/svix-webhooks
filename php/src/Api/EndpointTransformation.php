<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\EndpointTransformationOut;
use Svix\Models\EndpointTransformationPatch;
use Svix\Request\SvixHttpClient;

class EndpointTransformation
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Get the transformation code associated with this endpoint.
     *
     * @throws ApiException
     */
    public function get(
        string $appId,
        string $endpointId,
    ): EndpointTransformationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/transformation");
        $res = $this->client->send($request);

        return EndpointTransformationOut::fromJson($res);
    }

    /**
     * Set or unset the transformation code associated with this endpoint.
     *
     * @throws ApiException
     */
    public function patch(
        string $appId,
        string $endpointId,
        EndpointTransformationPatch $endpointTransformationPatch,
    ): void {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/endpoint/{$endpointId}/transformation");
        $request->setBody(json_encode($endpointTransformationPatch));
        $res = $this->client->sendNoResponseBody($request);
    }
}
