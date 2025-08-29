<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\BackgroundTaskOut;
use Svix\Models\ListResponseBackgroundTaskOut;
use Svix\Request\SvixHttpClient;

class BackgroundTask
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /** List background tasks executed in the past 90 days. */
    public function list(
        ?BackgroundTaskListOptions $options = null,
    ): ListResponseBackgroundTaskOut {
        $request = $this->client->newReq('GET', '/api/v1/background-task');
        if (null !== $options) {
            $request->setQueryParam('status', $options->status);
            $request->setQueryParam('task', $options->task);
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('order', $options->order);
        }
        $res = $this->client->send($request);

        return ListResponseBackgroundTaskOut::fromJson($res);
    }

    /** Get a background task by ID. */
    public function get(
        string $taskId,
    ): BackgroundTaskOut {
        $request = $this->client->newReq('GET', "/api/v1/background-task/{$taskId}");
        $res = $this->client->send($request);

        return BackgroundTaskOut::fromJson($res);
    }
}
