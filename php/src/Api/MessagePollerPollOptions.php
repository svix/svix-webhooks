<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class MessagePollerPollOptions
{
    public function __construct(
        public readonly ?int $limit = null,
        public readonly ?string $iterator = null,
        public readonly ?string $eventType = null,
        public readonly ?string $channel = null,
        public readonly ?\DateTimeImmutable $after = null,
    ) {
    }
}
