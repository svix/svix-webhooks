<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Request\SvixHttpClient;

class Destination
{
    public DestinationAutoconfig $autoconfig;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->autoconfig = new DestinationAutoconfig($client);
    }
}
