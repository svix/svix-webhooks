<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Request\SvixHttpClient;

class OperationalWebhook
{
    public OperationalWebhookEndpoint $endpoint;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->endpoint = new OperationalWebhookEndpoint($client);
    }
}
