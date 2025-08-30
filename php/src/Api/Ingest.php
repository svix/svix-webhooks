<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\DashboardAccessOut;
use Svix\Models\IngestSourceConsumerPortalAccessIn;
use Svix\Request\SvixHttpClient;

class Ingest
{
    public IngestEndpoint $endpoint;
    public IngestSource $source;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->endpoint = new IngestEndpoint($client);
        $this->source = new IngestSource($client);
    }

    /** Get access to the Ingest Source Consumer Portal. */
    public function dashboard(
        string $sourceId,
        IngestSourceConsumerPortalAccessIn $ingestSourceConsumerPortalAccessIn,
        ?IngestDashboardOptions $options = null,
    ): DashboardAccessOut {
        $request = $this->client->newReq('POST', "/ingest/api/v1/source/{$sourceId}/dashboard");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($ingestSourceConsumerPortalAccessIn));
        $res = $this->client->send($request);

        return DashboardAccessOut::fromJson($res);
    }
}
