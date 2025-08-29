<?php

declare(strict_types=1);

namespace Svix;

class SvixOptions
{
    public function __construct(
        public bool $debug = false,
        public ?string $serverUrl = null,
        public ?int $timeoutSecs = null,
        public ?int $numRetries = null,
        public ?array $retryScheduleMs = null,
        public ?string $proxyAddress = null,
    ) {}
}
