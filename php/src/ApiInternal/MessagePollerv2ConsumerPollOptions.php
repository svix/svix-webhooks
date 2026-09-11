<?php

declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Models\StartingPosition;

class MessagePollerv2ConsumerPollOptions
{
    public function __construct(
        public readonly ?int $limit = null,
        public readonly ?int $leaseDurationMs = null,
        public readonly ?StartingPosition $startingPosition = null,
    ) {
    }
}
