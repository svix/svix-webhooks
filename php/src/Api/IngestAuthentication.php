<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\AppPortalAccessOut;
use Svix\Models\IngestSourceConsumerPortalAccessIn;
use Svix\Request\SvixHttpClient;

class IngestAuthentication
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Get access to the Ingest Source Consumer Portal.
     *
     * @throws ApiException
     */
    public function consumerPortalAccess(
        string $sourceId,
        IngestSourceConsumerPortalAccessIn $ingestSourceConsumerPortalAccessIn,
        IngestAuthenticationConsumerPortalAccessOptions $options = new IngestAuthenticationConsumerPortalAccessOptions(),
    ): AppPortalAccessOut {
        $request = $this->client->newReq('POST', "/ingest/api/v1/source/{$sourceId}/dashboard");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($ingestSourceConsumerPortalAccessIn));
        $res = $this->client->send($request);

        return AppPortalAccessOut::fromJson($res);
    }
}
