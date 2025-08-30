<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\BackgroundTaskStatus;
use Svix\Models\BackgroundTaskType;
use Svix\Models\Ordering;

class BackgroundTaskListOptions
{
    public function __construct(
        public readonly ?BackgroundTaskStatus $status = null,
        public readonly ?BackgroundTaskType $task = null,
        public readonly ?int $limit = null,
        public readonly ?string $iterator = null,
        public readonly ?Ordering $order = null,
    ) {
    }
}
