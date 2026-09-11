<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\AutoConfigOut;
use Svix\Models\RotateSubscriptionIn2;
use Svix\Request\SvixHttpClient;

class Autoconfig
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Create an AutoConfig subscription.
     *
     * @throws ApiException
     */
    public function create(
        string $appId,
        AutoconfigCreateOptions $options = new AutoconfigCreateOptions(),
    ): AutoConfigOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/autoconfig");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $res = $this->client->send($request);

        return AutoConfigOut::fromJson($res);
    }

    /**
     * Rotate the auth token and signing secret for an AutoConfig subscription.
     *
     * @throws ApiException
     */
    public function rotate(
        string $appId,
        string $autoconfigId,
        RotateSubscriptionIn2 $rotateSubscriptionIn2,
        AutoconfigRotateOptions $options = new AutoconfigRotateOptions(),
    ): AutoConfigOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}/rotate");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($rotateSubscriptionIn2));
        $res = $this->client->send($request);

        return AutoConfigOut::fromJson($res);
    }
}
