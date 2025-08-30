<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class MessageListOptions
{
    public function __construct(
        public readonly ?int $limit = null,
        public readonly ?string $iterator = null,
        public readonly ?string $channel = null,
        public readonly ?\DateTimeImmutable $before = null,
        public readonly ?\DateTimeImmutable $after = null,
        public readonly ?bool $withContent = null,
        public readonly ?string $tag = null,
        public readonly ?array $eventTypes = null,
    ) {
    }
}
