<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\DestinationIn;
use Svix\Models\DestinationOut;
use Svix\Models\DestinationPatch;
use Svix\Models\ListResponseDestinationOut;
use Svix\Request\SvixHttpClient;

class Destination
{
    public DestinationTransformation $transformation;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->transformation = new DestinationTransformation($client);
    }

    /**
     * List of all the application's destinations.
     *
     * @throws ApiException
     */
    public function list(
        string $appId,
        DestinationListOptions $options = new DestinationListOptions(),
    ): ListResponseDestinationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/destination");
        $request->setQueryParam('limit', $options->limit);
        $request->setQueryParam('iterator', $options->iterator);
        $request->setQueryParam('order', $options->order);
        $res = $this->client->send($request);

        return ListResponseDestinationOut::fromJson($res);
    }

    /**
     * Creates a new destination.
     *
     * @throws ApiException
     */
    public function create(
        string $appId,
        DestinationIn $destinationIn,
        DestinationCreateOptions $options = new DestinationCreateOptions(),
    ): DestinationOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/destination");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($destinationIn));
        $res = $this->client->send($request);

        return DestinationOut::fromJson($res);
    }

    /**
     * Get a destination by id or uid.
     *
     * @throws ApiException
     */
    public function get(
        string $appId,
        string $destinationId,
    ): DestinationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/destination/{$destinationId}");
        $res = $this->client->send($request);

        return DestinationOut::fromJson($res);
    }

    /**
     * Create or update a destination.
     *
     * @throws ApiException
     */
    public function upsert(
        string $appId,
        string $destinationId,
        DestinationIn $destinationIn,
    ): DestinationOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}/destination/{$destinationId}");
        $request->setBody(json_encode($destinationIn));
        $res = $this->client->send($request);

        return DestinationOut::fromJson($res);
    }

    /**
     * Delete a destination.
     *
     * @throws ApiException
     */
    public function delete(
        string $appId,
        string $destinationId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/destination/{$destinationId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /**
     * Partially update a destination.
     *
     * @throws ApiException
     */
    public function patch(
        string $appId,
        string $destinationId,
        DestinationPatch $destinationPatch,
    ): DestinationOut {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}/destination/{$destinationId}");
        $request->setBody(json_encode($destinationPatch));
        $res = $this->client->send($request);

        return DestinationOut::fromJson($res);
    }
}
