<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Models\EndpointTransformationIn;
use Svix\Request\SvixHttpClient;

class Endpoint
{
    public EndpointAutoConfig $autoConfig;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->autoConfig = new EndpointAutoConfig($client);
    }

    /** This operation was renamed to `set-transformation`. */
    public function transformationPartialUpdate(
        string $appId,
        string $endpointId,
        EndpointTransformationIn $endpointTransformationIn,
    ): void {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/endpoint/{$endpointId}/transformation");
        $request->setBody(json_encode($endpointTransformationIn));
        $res = $this->client->sendNoResponseBody($request);
    }
}
