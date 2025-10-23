<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\AggregateEventTypesOut;
use Svix\Models\AppUsageStatsIn;
use Svix\Models\AppUsageStatsOut;
use Svix\Request\SvixHttpClient;

class Statistics
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Creates a background task to calculate the message destinations for all applications in the environment.
     *
     * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
     * retrieve the results of the operation.
     *
     * The completed background task will return a payload like the following:
     * ```json
     * {
     *   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
     *   "status": "finished",
     *   "task": "application.stats",
     *   "data": {
     *     "appStats": [
     *       {
     *         "messageDestinations": 2,
     *         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
     *         "appUid": null
     *       }
     *     ]
     *   }
     * }
     * ```
     */
    public function aggregateAppStats(
        AppUsageStatsIn $appUsageStatsIn,
        ?StatisticsAggregateAppStatsOptions $options = null,
    ): AppUsageStatsOut {
        $request = $this->client->newReq('POST', '/api/v1/stats/usage/app');
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($appUsageStatsIn));
        $res = $this->client->send($request);

        return AppUsageStatsOut::fromJson($res);
    }

    /**
     * Creates a background task to calculate the listed event types for all apps in the organization.
     *
     * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
     * retrieve the results of the operation.
     *
     * The completed background task will return a payload like the following:
     * ```json
     * {
     *   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
     *   "status": "finished",
     *   "task": "event-type.aggregate",
     *   "data": {
     *     "event_types": [
     *       {
     *         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
     *         "explicitlySubscribedEventTypes": ["user.signup", "user.deleted"],
     *         "hasCatchAllEndpoint": false
     *       }
     *     ]
     *   }
     * }
     * ```
     */
    public function aggregateEventTypes(
    ): AggregateEventTypesOut {
        $request = $this->client->newReq('PUT', '/api/v1/stats/usage/event-types');
        $res = $this->client->send($request);

        return AggregateEventTypesOut::fromJson($res);
    }
}
