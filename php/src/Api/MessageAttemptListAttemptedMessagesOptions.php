<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\MessageStatus;

class MessageAttemptListAttemptedMessagesOptions
{
    public function __construct(
        public readonly ?int $limit = null,
        public readonly ?string $iterator = null,
        public readonly ?string $channel = null,
        public readonly ?string $tag = null,
        public readonly ?MessageStatus $status = null,
        public readonly ?\DateTimeImmutable $before = null,
        public readonly ?\DateTimeImmutable $after = null,
        public readonly ?bool $withContent = null,
        public readonly ?array $eventTypes = null,
    ) {
    }
}
