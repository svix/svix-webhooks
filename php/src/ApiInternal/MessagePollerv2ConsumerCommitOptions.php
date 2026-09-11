<?php

declare(strict_types=1);

namespace Svix\ApiInternal;

class MessagePollerv2ConsumerCommitOptions
{
    public function __construct(
        public readonly ?string $idempotencyKey = null,
    ) {
    }
}
