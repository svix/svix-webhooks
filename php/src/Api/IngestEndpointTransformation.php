<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\IngestEndpointTransformationOut;
use Svix\Models\IngestEndpointTransformationPatch;
use Svix\Request\SvixHttpClient;

class IngestEndpointTransformation
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Get the transformation code associated with this ingest endpoint.
     *
     * @throws ApiException
     */
    public function transformation(
        string $sourceId,
        string $endpointId,
    ): IngestEndpointTransformationOut {
        $request = $this->client->newReq('GET', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/transformation");
        $res = $this->client->send($request);

        return IngestEndpointTransformationOut::fromJson($res);
    }

    /**
     * Set or unset the transformation code associated with this ingest endpoint.
     *
     * @throws ApiException
     */
    public function patch(
        string $sourceId,
        string $endpointId,
        IngestEndpointTransformationPatch $ingestEndpointTransformationPatch,
    ): void {
        $request = $this->client->newReq('PATCH', "/ingest/api/v1/source/{$sourceId}/endpoint/{$endpointId}/transformation");
        $request->setBody(json_encode($ingestEndpointTransformationPatch));
        $res = $this->client->sendNoResponseBody($request);
    }
}
