<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\Ordering;

class StreamEventTypeListOptions
{
    public function __construct(
        public readonly ?int $limit = null,
        public readonly ?string $iterator = null,
        public readonly ?Ordering $order = null,
        public readonly ?bool $includeArchived = null,
    ) {
    }
}
