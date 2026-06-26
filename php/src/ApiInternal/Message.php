<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Request\SvixHttpClient;

class Message
{
    public MessagePollerv2 $pollerv2;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->pollerv2 = new MessagePollerv2($client);
    }
}
