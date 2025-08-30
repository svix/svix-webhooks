<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

class MessageCreateOptions
{
    public function __construct(
        public readonly ?bool $withContent = null,
        public readonly ?string $idempotencyKey = null,
    ) {
    }
}
