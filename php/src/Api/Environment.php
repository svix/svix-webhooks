<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\EnvironmentIn;
use Svix\Models\EnvironmentOut;
use Svix\Request\SvixHttpClient;

class Environment
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Download a JSON file containing all org-settings and event types.
     *
     * Note that the schema for [`EnvironmentOut`] is subject to change. The fields
     * herein are provided for convenience but should be treated as JSON blobs.
     */
    public function export(
        ?EnvironmentExportOptions $options = null,
    ): EnvironmentOut {
        $request = $this->client->newReq('POST', '/api/v1/environment/export');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->send($request);

        return EnvironmentOut::fromJson($res);
    }

    /**
     * Import a configuration into the active organization.
     *
     * It doesn't delete anything, only adds / updates what was passed to it.
     *
     * Note that the schema for [`EnvironmentIn`] is subject to change. The fields
     * herein are provided for convenience but should be treated as JSON blobs.
     */
    public function import(
        EnvironmentIn $environmentIn,
        ?EnvironmentImportOptions $options = null,
    ): void {
        $request = $this->client->newReq('POST', '/api/v1/environment/import');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($environmentIn));
        $res = $this->client->sendNoResponseBody($request);
    }
}
