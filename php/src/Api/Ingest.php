<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Request\SvixHttpClient;

class Ingest
{
    public IngestAuthentication $authentication;
    public IngestEndpoint $endpoint;
    public IngestSource $source;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->authentication = new IngestAuthentication($client);
        $this->endpoint = new IngestEndpoint($client);
        $this->source = new IngestSource($client);
    }
}
