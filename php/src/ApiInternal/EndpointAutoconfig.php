<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Exception\ApiException;
use Svix\Models\EndpointIn;
use Svix\Models\EndpointOut;
use Svix\Request\SvixHttpClient;

class EndpointAutoconfig
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Create or update the HTTP endpoint for an AutoConfig subscription.
     *
     * @throws ApiException
     */
    public function subscribe(
        string $appId,
        string $autoconfigId,
        EndpointIn $endpointIn,
    ): EndpointOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}/endpoint");
        $request->setBody(json_encode($endpointIn));
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }
}
