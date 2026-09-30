<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\AutoConfigOut;
use Svix\Models\CreateAutoConfigSubscriptionIn;
use Svix\Models\RotateSubscriptionIn2;
use Svix\Request\SvixHttpClient;

class AutoconfigSubscription
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
        CreateAutoConfigSubscriptionIn $createAutoConfigSubscriptionIn,
        AutoconfigSubscriptionCreateOptions $options = new AutoconfigSubscriptionCreateOptions(),
    ): AutoConfigOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/autoconfig");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($createAutoConfigSubscriptionIn));
        $res = $this->client->send($request);

        return AutoConfigOut::fromJson($res);
    }

    /**
     * Delete an AutoConfig subscription. This also invalidates its auth token.
     *
     * @throws ApiException
     */
    public function delete(
        string $appId,
        string $autoconfigId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}");
        $res = $this->client->sendNoResponseBody($request);
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
        AutoconfigSubscriptionRotateOptions $options = new AutoconfigSubscriptionRotateOptions(),
    ): AutoConfigOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/autoconfig/{$autoconfigId}/rotate");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($rotateSubscriptionIn2));
        $res = $this->client->send($request);

        return AutoConfigOut::fromJson($res);
    }
}
