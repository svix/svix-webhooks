<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Request\SvixHttpClient;

class Health
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** Verify the API server is up and running. */
    public function get(
    ): void {
        $request = $this->client->newReq('GET', '/api/v1/health');
        $res = $this->client->sendNoResponseBody($request);
    }
}
