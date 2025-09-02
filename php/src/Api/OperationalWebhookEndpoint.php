<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ListResponseOperationalWebhookEndpointOut;
use Svix\Models\OperationalWebhookEndpointHeadersIn;
use Svix\Models\OperationalWebhookEndpointHeadersOut;
use Svix\Models\OperationalWebhookEndpointIn;
use Svix\Models\OperationalWebhookEndpointOut;
use Svix\Models\OperationalWebhookEndpointSecretIn;
use Svix\Models\OperationalWebhookEndpointSecretOut;
use Svix\Models\OperationalWebhookEndpointUpdate;
use Svix\Request\SvixHttpClient;

class OperationalWebhookEndpoint
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List operational webhook endpoints. */
    public function list(
        ?OperationalWebhookEndpointListOptions $options = null,
    ): ListResponseOperationalWebhookEndpointOut {
        $request = $this->client->newReq('GET', '/api/v1/operational-webhook/endpoint');
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseOperationalWebhookEndpointOut::fromJson($res);
    }

    /** Create an operational webhook endpoint. */
    public function create(
        OperationalWebhookEndpointIn $operationalWebhookEndpointIn,
        ?OperationalWebhookEndpointCreateOptions $options = null,
    ): OperationalWebhookEndpointOut {
        $request = $this->client->newReq('POST', '/api/v1/operational-webhook/endpoint');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($operationalWebhookEndpointIn));
        $res = $this->client->send($request);

        return OperationalWebhookEndpointOut::fromJson($res);
    }

    /** Get an operational webhook endpoint. */
    public function get(
        string $endpointId,
    ): OperationalWebhookEndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/operational-webhook/endpoint/{$endpointId}");
        $res = $this->client->send($request);

        return OperationalWebhookEndpointOut::fromJson($res);
    }

    /** Update an operational webhook endpoint. */
    public function update(
        string $endpointId,
        OperationalWebhookEndpointUpdate $operationalWebhookEndpointUpdate,
    ): OperationalWebhookEndpointOut {
        $request = $this->client->newReq('PUT', "/api/v1/operational-webhook/endpoint/{$endpointId}");
        $request->setBody(json_encode($operationalWebhookEndpointUpdate));
        $res = $this->client->send($request);

        return OperationalWebhookEndpointOut::fromJson($res);
    }

    /** Delete an operational webhook endpoint. */
    public function delete(
        string $endpointId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/operational-webhook/endpoint/{$endpointId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Get the additional headers to be sent with the operational webhook. */
    public function getHeaders(
        string $endpointId,
    ): OperationalWebhookEndpointHeadersOut {
        $request = $this->client->newReq('GET', "/api/v1/operational-webhook/endpoint/{$endpointId}/headers");
        $res = $this->client->send($request);

        return OperationalWebhookEndpointHeadersOut::fromJson($res);
    }

    /** Set the additional headers to be sent with the operational webhook. */
    public function updateHeaders(
        string $endpointId,
        OperationalWebhookEndpointHeadersIn $operationalWebhookEndpointHeadersIn,
    ): void {
        $request = $this->client->newReq('PUT', "/api/v1/operational-webhook/endpoint/{$endpointId}/headers");
        $request->setBody(json_encode($operationalWebhookEndpointHeadersIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Get an operational webhook endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook.
     * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public function getSecret(
        string $endpointId,
    ): OperationalWebhookEndpointSecretOut {
        $request = $this->client->newReq('GET', "/api/v1/operational-webhook/endpoint/{$endpointId}/secret");
        $res = $this->client->send($request);

        return OperationalWebhookEndpointSecretOut::fromJson($res);
    }

    /**
     * Rotates an operational webhook endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    public function rotateSecret(
        string $endpointId,
        OperationalWebhookEndpointSecretIn $operationalWebhookEndpointSecretIn,
        ?OperationalWebhookEndpointRotateSecretOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/operational-webhook/endpoint/{$endpointId}/secret/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($operationalWebhookEndpointSecretIn));
        $res = $this->client->sendNoResponseBody($request);
    }
}
