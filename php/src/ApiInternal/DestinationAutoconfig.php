<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Exception\ApiException;
use Svix\Models\DestinationIn;
use Svix\Models\DestinationOut;
use Svix\Request\SvixHttpClient;

class DestinationAutoconfig
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Create or update the destination for an AutoConfig subscription.
     *
     * @throws ApiException
     */
    public function subscribe(
        string $appId,
        string $autoconfigId,
        DestinationIn $destinationIn,
    ): DestinationOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}/destination");
        $request->setBody(json_encode($destinationIn));
        $res = $this->client->send($request);

        return DestinationOut::fromJson($res);
    }
}
