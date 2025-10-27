<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ConnectorIn;
use Svix\Models\ConnectorOut;
use Svix\Models\ConnectorPatch;
use Svix\Models\ConnectorUpdate;
use Svix\Models\ListResponseConnectorOut;
use Svix\Request\SvixHttpClient;

class Connector
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List all connectors for an application. */
    public function list(
        ?ConnectorListOptions $options = null,
    ): ListResponseConnectorOut {
        $request = $this->client->newReq('GET', '/api/v1/connector');
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseConnectorOut::fromJson($res);
    }

    /** Create a new connector. */
    public function create(
        ConnectorIn $connectorIn,
        ?ConnectorCreateOptions $options = null,
    ): ConnectorOut {
        $request = $this->client->newReq('POST', '/api/v1/connector');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($connectorIn));
        $res = $this->client->send($request);

        return ConnectorOut::fromJson($res);
    }

    /** Get a connector. */
    public function get(
        string $connectorId,
    ): ConnectorOut {
        $request = $this->client->newReq('GET', "/api/v1/connector/{$connectorId}");
        $res = $this->client->send($request);

        return ConnectorOut::fromJson($res);
    }

    /** Update a connector. */
    public function update(
        string $connectorId,
        ConnectorUpdate $connectorUpdate,
    ): ConnectorOut {
        $request = $this->client->newReq('PUT', "/api/v1/connector/{$connectorId}");
        $request->setBody(json_encode($connectorUpdate));
        $res = $this->client->send($request);

        return ConnectorOut::fromJson($res);
    }

    /** Delete a connector. */
    public function delete(
        string $connectorId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/connector/{$connectorId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update a connector. */
    public function patch(
        string $connectorId,
        ConnectorPatch $connectorPatch,
    ): ConnectorOut {
        $request = $this->client->newReq('PATCH', "/api/v1/connector/{$connectorId}");
        $request->setBody(json_encode($connectorPatch));
        $res = $this->client->send($request);

        return ConnectorOut::fromJson($res);
    }
}
