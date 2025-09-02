<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class EndpointGetStatsOptions
{
    public function __construct(
        public readonly ?\DateTimeImmutable $since = null,
        public readonly ?\DateTimeImmutable $until = null,
    ) {
    }
}
