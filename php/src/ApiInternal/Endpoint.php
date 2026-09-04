<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Exception\ApiException;
use Svix\Models\EndpointTransformationIn;
use Svix\Request\SvixHttpClient;

class Endpoint
{
    public EndpointAutoConfigDeprecated $autoConfigDeprecated;
    public EndpointAutoconfig $autoconfig;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->autoConfigDeprecated = new EndpointAutoConfigDeprecated($client);
        $this->autoconfig = new EndpointAutoconfig($client);
    }

    /**
     * This operation was renamed to `set-transformation`.
     *
     * @throws ApiException
     */
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
