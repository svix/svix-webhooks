<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ApplicationIn;
use Svix\Models\ApplicationOut;
use Svix\Models\ApplicationPatch;
use Svix\Models\ListResponseApplicationOut;
use Svix\Request\SvixHttpClient;

class Application
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List of all the organization's applications. */
    public function list(
        ?ApplicationListOptions $options = null,
    ): ListResponseApplicationOut {
        $request = $this->client->newReq('GET', '/api/v1/app');
        if (null !== $options) {
            $request->setQueryParam('exclude_apps_with_no_endpoints', $options->excludeAppsWithNoEndpoints);
            $request->setQueryParam('exclude_apps_with_disabled_endpoints', $options->excludeAppsWithDisabledEndpoints);
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseApplicationOut::fromJson($res);
    }

    /** Create a new application. */
    public function create(
        ApplicationIn $applicationIn,
        ?ApplicationCreateOptions $options = null,
    ): ApplicationOut {
        $request = $this->client->newReq('POST', '/api/v1/app');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($applicationIn));
        $res = $this->client->send($request);

        return ApplicationOut::fromJson($res);
    }

    /** Get or create a new application */
    public function getOrCreate(
        ApplicationIn $applicationIn,
        ?ApplicationCreateOptions $options = null,
    ): ApplicationOut {
        $request = $this->client->newReq('POST', '/api/v1/app');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setQueryParam('get_if_exists', 'true');
        $request->setBody(json_encode($applicationIn));
        $res = $this->client->send($request);

        return ApplicationOut::fromJson($res);
    }

    /** Get an application. */
    public function get(
        string $appId,
    ): ApplicationOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}");
        $res = $this->client->send($request);

        return ApplicationOut::fromJson($res);
    }

    /** Update an application. */
    public function update(
        string $appId,
        ApplicationIn $applicationIn,
    ): ApplicationOut {
        $request = $this->client->newReq('PUT', "/api/v1/app/{$appId}");
        $request->setBody(json_encode($applicationIn));
        $res = $this->client->send($request);

        return ApplicationOut::fromJson($res);
    }

    /** Delete an application. */
    public function delete(
        string $appId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}");
        $res = $this->client->sendNoResponseBody($request);
    }

    /** Partially update an application. */
    public function patch(
        string $appId,
        ApplicationPatch $applicationPatch,
    ): ApplicationOut {
        $request = $this->client->newReq('PATCH', "/api/v1/app/{$appId}");
        $request->setBody(json_encode($applicationPatch));
        $res = $this->client->send($request);

        return ApplicationOut::fromJson($res);
    }
}
