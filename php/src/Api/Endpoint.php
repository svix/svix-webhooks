<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\EndpointHeadersIn;
use Svix\Models\EndpointHeadersOut;
use Svix\Models\EndpointHeadersPatchIn;
use Svix\Models\EndpointIn;
use Svix\Models\EndpointOut;
use Svix\Models\EndpointPatch;
use Svix\Models\EndpointSecretOut;
use Svix\Models\EndpointSecretRotateIn;
use Svix\Models\EndpointStats;
use Svix\Models\EndpointTransformationIn;
use Svix\Models\EndpointTransformationOut;
use Svix\Models\EndpointTransformationPatch;
use Svix\Models\EndpointUpdate;
use Svix\Models\EventExampleIn;
use Svix\Models\ListResponseEndpointOut;
use Svix\Models\MessageOut;
use Svix\Models\RecoverIn;
use Svix\Models\RecoverOut;
use Svix\Models\ReplayIn;
use Svix\Models\ReplayOut;
use Svix\Request\SvixHttpClient;

class Endpoint
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List the application's endpoints. */
    public function list(
        string $appId,
        ?EndpointListOptions $options = null,
    ): ListResponseEndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseEndpointOut::fromJson($res);
    }

    /**
     * Create a new endpoint for the application.
     *
     * When `secret` is `null` the secret is automatically generated (recommended).
     */
    public function create(
        string $appId,
        EndpointIn $endpointIn,
        ?EndpointCreateOptions $options = null,
    ): EndpointOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/endpoint");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($endpointIn));
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }

    /** Get an endpoint. */
    public function get(
        string $appId,
        string $endpointId,
    ): EndpointOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}");
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }

    /** Update an endpoint. */
    public function update(
        string $appId,
        string $endpointId,
        EndpointUpdate $endpointUpdate,
    ): EndpointOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/endpoint/{$endpointId}");
        $request->setBody(json_encode($endpointUpdate));
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }

    /** Delete an endpoint. */
    public function delete(
        string $appId,
        string $endpointId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/endpoint/{$endpointId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update an endpoint. */
    public function patch(
        string $appId,
        string $endpointId,
        EndpointPatch $endpointPatch,
    ): EndpointOut {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/endpoint/{$endpointId}");
        $request->setBody(json_encode($endpointPatch));
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }

    /** Get the additional headers to be sent with the webhook. */
    public function getHeaders(
        string $appId,
        string $endpointId,
    ): EndpointHeadersOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/headers");
        $res = $this->client->send($request);

        return EndpointHeadersOut::fromJson($res);
    }

    /** Set the additional headers to be sent with the webhook. */
    public function updateHeaders(
        string $appId,
        string $endpointId,
        EndpointHeadersIn $endpointHeadersIn,
    ): void {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/endpoint/{$endpointId}/headers");
        $request->setBody(json_encode($endpointHeadersIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially set the additional headers to be sent with the webhook. */
    public function patchHeaders(
        string $appId,
        string $endpointId,
        EndpointHeadersPatchIn $endpointHeadersPatchIn,
    ): void {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/endpoint/{$endpointId}/headers");
        $request->setBody(json_encode($endpointHeadersPatchIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Resend all failed messages since a given time.
     *
     * Messages that were sent successfully, even if failed initially, are not resent.
     */
    public function recover(
        string $appId,
        string $endpointId,
        RecoverIn $recoverIn,
        ?EndpointRecoverOptions $options = null,
    ): RecoverOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/endpoint/{$endpointId}/recover");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($recoverIn));
        $res = $this->client->send($request);

        return RecoverOut::fromJson($res);
    }

    /**
     * Replays messages to the endpoint.
     *
     * Only messages that were created after `since` will be sent.
     * Messages that were previously sent to the endpoint are not resent.
     */
    public function replayMissing(
        string $appId,
        string $endpointId,
        ReplayIn $replayIn,
        ?EndpointReplayMissingOptions $options = null,
    ): ReplayOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/endpoint/{$endpointId}/replay-missing");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($replayIn));
        $res = $this->client->send($request);

        return ReplayOut::fromJson($res);
    }

    /**
     * Get the endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook.
     * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public function getSecret(
        string $appId,
        string $endpointId,
    ): EndpointSecretOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/secret");
        $res = $this->client->send($request);

        return EndpointSecretOut::fromJson($res);
    }

    /**
     * Rotates the endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    public function rotateSecret(
        string $appId,
        string $endpointId,
        EndpointSecretRotateIn $endpointSecretRotateIn,
        ?EndpointRotateSecretOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/endpoint/{$endpointId}/secret/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($endpointSecretRotateIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Send an example message for an event. */
    public function sendExample(
        string $appId,
        string $endpointId,
        EventExampleIn $eventExampleIn,
        ?EndpointSendExampleOptions $options = null,
    ): MessageOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/endpoint/{$endpointId}/send-example");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($eventExampleIn));
        $res = $this->client->send($request);

        return MessageOut::fromJson($res);
    }

    /** Get basic statistics for the endpoint. */
    public function getStats(
        string $appId,
        string $endpointId,
        ?EndpointGetStatsOptions $options = null,
    ): EndpointStats {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/stats");
        if (null !== $options) {
            $request->setQueryParam('since', $options->since);
            $request->setQueryParam('until', $options->until);
        }
        $res = $this->client->send($request);

        return EndpointStats::fromJson($res);
    }

    /** Get the transformation code associated with this endpoint. */
    public function transformationGet(
        string $appId,
        string $endpointId,
    ): EndpointTransformationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/endpoint/{$endpointId}/transformation");
        $res = $this->client->send($request);

        return EndpointTransformationOut::fromJson($res);
    }

    /** Set or unset the transformation code associated with this endpoint. */
    public function patchTransformation(
        string $appId,
        string $endpointId,
        EndpointTransformationPatch $endpointTransformationPatch,
    ): void {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/endpoint/{$endpointId}/transformation");
        $request->setBody(json_encode($endpointTransformationPatch));
        $res = $this->client->sendNoResponseBody($request);
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
