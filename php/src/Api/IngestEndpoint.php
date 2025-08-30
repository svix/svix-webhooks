<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\IngestEndpointHeadersIn;
use Svix\Models\IngestEndpointHeadersOut;
use Svix\Models\IngestEndpointIn;
use Svix\Models\IngestEndpointOut;
use Svix\Models\IngestEndpointSecretIn;
use Svix\Models\IngestEndpointSecretOut;
use Svix\Models\IngestEndpointTransformationOut;
use Svix\Models\IngestEndpointTransformationPatch;
use Svix\Models\IngestEndpointUpdate;
use Svix\Models\ListResponseIngestEndpointOut;
use Svix\Request\SvixHttpClient;

class IngestEndpoint
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List ingest endpoints. */
    public function list(
        string $sourceId,
        ?IngestEndpointListOptions $options = null,
    ): ListResponseIngestEndpointOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseIngestEndpointOut::fromJson($res);
    }

    /** Create an ingest endpoint. */
    public function create(
        string $sourceId,
        IngestEndpointIn $ingestEndpointIn,
        ?IngestEndpointCreateOptions $options = null,
    ): IngestEndpointOut {
        $request = $this->client->newReq('POST', "/ingest/api/v1/source/{$sourceId}/endpoint");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($ingestEndpointIn));
        $res = $this->client->send($request);

        return IngestEndpointOut::fromJson($res);
    }

    /** Get an ingest endpoint. */
    public function get(
        string $sourceId,
        string $endpointId,
    ): IngestEndpointOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}");
        $res = $this->client->send($request);

        return IngestEndpointOut::fromJson($res);
    }

    /** Update an ingest endpoint. */
    public function update(
        string $sourceId,
        string $endpointId,
        IngestEndpointUpdate $ingestEndpointUpdate,
    ): IngestEndpointOut {
        $request = $this->client->newReq('PUT', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}");
        $request->setBody(json_encode($ingestEndpointUpdate));
        $res = $this->client->send($request);

        return IngestEndpointOut::fromJson($res);
    }

    /** Delete an ingest endpoint. */
    public function delete(
        string $sourceId,
        string $endpointId,
    ): void {
        $request = $this->client->newReq('DELETE', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Get the additional headers to be sent with the ingest. */
    public function getHeaders(
        string $sourceId,
        string $endpointId,
    ): IngestEndpointHeadersOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/headers");
        $res = $this->client->send($request);

        return IngestEndpointHeadersOut::fromJson($res);
    }

    /** Set the additional headers to be sent to the endpoint. */
    public function updateHeaders(
        string $sourceId,
        string $endpointId,
        IngestEndpointHeadersIn $ingestEndpointHeadersIn,
    ): void {
        $request = $this->client->newReq('PUT', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/headers");
        $request->setBody(json_encode($ingestEndpointHeadersIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Get an ingest endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook.
     * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public function getSecret(
        string $sourceId,
        string $endpointId,
    ): IngestEndpointSecretOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/secret");
        $res = $this->client->send($request);

        return IngestEndpointSecretOut::fromJson($res);
    }

    /**
     * Rotates an ingest endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    public function rotateSecret(
        string $sourceId,
        string $endpointId,
        IngestEndpointSecretIn $ingestEndpointSecretIn,
        ?IngestEndpointRotateSecretOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/secret/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($ingestEndpointSecretIn));
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Get the transformation code associated with this ingest endpoint. */
    public function getTransformation(
        string $sourceId,
        string $endpointId,
    ): IngestEndpointTransformationOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/transformation");
        $res = $this->client->send($request);

        return IngestEndpointTransformationOut::fromJson($res);
    }

    /** Set or unset the transformation code associated with this ingest endpoint. */
    public function setTransformation(
        string $sourceId,
        string $endpointId,
        IngestEndpointTransformationPatch $ingestEndpointTransformationPatch,
    ): void {
        $request = $this->client->newReq('PATCH', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/transformation");
        $request->setBody(json_encode($ingestEndpointTransformationPatch));
        $res = $this->client->sendNoResponseBody($request);
    }
}
