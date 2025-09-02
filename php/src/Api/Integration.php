<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\IntegrationIn;
use Svix\Models\IntegrationKeyOut;
use Svix\Models\IntegrationOut;
use Svix\Models\IntegrationUpdate;
use Svix\Models\ListResponseIntegrationOut;
use Svix\Request\SvixHttpClient;

class Integration
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List the application's integrations. */
    public function list(
        string $appId,
        ?IntegrationListOptions $options = null,
    ): ListResponseIntegrationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/integration");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseIntegrationOut::fromJson($res);
    }

    /** Create an integration. */
    public function create(
        string $appId,
        IntegrationIn $integrationIn,
        ?IntegrationCreateOptions $options = null,
    ): IntegrationOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/integration");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($integrationIn));
        $res = $this->client->send($request);

        return IntegrationOut::fromJson($res);
    }

    /** Get an integration. */
    public function get(
        string $appId,
        string $integId,
    ): IntegrationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/integration/{$integId}");
        $res = $this->client->send($request);

        return IntegrationOut::fromJson($res);
    }

    /** Update an integration. */
    public function update(
        string $appId,
        string $integId,
        IntegrationUpdate $integrationUpdate,
    ): IntegrationOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/integration/{$integId}");
        $request->setBody(json_encode($integrationUpdate));
        $res = $this->client->send($request);

        return IntegrationOut::fromJson($res);
    }

    /** Delete an integration. */
    public function delete(
        string $appId,
        string $integId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/integration/{$integId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Get an integration's key. */
    public function getKey(
        string $appId,
        string $integId,
    ): IntegrationKeyOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/integration/{$integId}/key");
        $res = $this->client->send($request);

        return IntegrationKeyOut::fromJson($res);
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    public function rotateKey(
        string $appId,
        string $integId,
        ?IntegrationRotateKeyOptions $options = null,
    ): IntegrationKeyOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/integration/{$integId}/key/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->send($request);

        return IntegrationKeyOut::fromJson($res);
    }
}
