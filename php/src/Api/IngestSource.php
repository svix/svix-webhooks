<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\RotateTokenOut;
use Svix\Request\SvixHttpClient;

class IngestSource
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Rotate the Ingest Source's Url Token.
     *
     * This will rotate the ingest source's token, which is used to
     * construct the unique `ingestUrl` for the source. Previous tokens
     * will remain valid for 48 hours after rotation. The token can be
     * rotated a maximum of three times within the 48-hour period.
     */
    public function rotateToken(
        string $sourceId,
        ?IngestSourceRotateTokenOptions $options = null,
    ): RotateTokenOut {
        $request = $this->client->newReq('POST', "/ingest/api/v1/source/{$sourceId}/token/rotate");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->send($request);

        return RotateTokenOut::fromJson($res);
    }
}
