<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class MessagePollerConsumerSeekOptions
{
    public function __construct(
        public readonly ?string $idempotencyKey = null,
    ) {
    }
}
