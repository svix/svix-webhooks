<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class EndpointGetAttemptStatsOptions
{
    public function __construct(
        public readonly ?\DateTimeImmutable $since = null,
        public readonly ?\DateTimeImmutable $until = null,
    ) {
    }
}
