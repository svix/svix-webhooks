<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Models\EndpointOut;
use Svix\Models\SubscribeIn;
use Svix\Request\SvixHttpClient;

class EndpointAutoConfig
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Update an auto-config endpoint by providing endpoint details. */
    public function update(
        string $appId,
        string $endpointId,
        SubscribeIn $subscribeIn,
    ): EndpointOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/endpoint/{$endpointId}/auto-config");
        $request->setBody(json_encode($subscribeIn));
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }
}
