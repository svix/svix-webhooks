<?php

declare(strict_types=1);

namespace Svix;

/**
 * Configuration options for the Svix client
 */
class SvixOptions
{
    public function __construct(
        /** Enable debug mode for additional logging and error information */
        public bool $debug = false,

        /** Custom server URL to override the default Svix API endpoint */
        public ?string $serverUrl = null,

        /** Request timeout in milliseconds */
        public ?int $timeoutMs = 30000,

        /** Number of retries
         *
         * The number of times the client will retry if a server-side error
         * or timeout is received.
         *
         * Default: 2 */
        public ?int $numRetries = 2,

        /** Retry Schedule in milliseconds
         *
         * List of delays to wait before each retry attempt.
         * Takes precedence over `numRetries`.
         */
        public ?array $retryScheduleMs = [60, 120, 240],

    ) {}

    public static function newDefault(string $token): SvixOptions
    {
        return new SvixOptions(
            debug: false,
            serverUrl: Utils::getServerUrlFromToken($token),
            timeoutMs: 30000,
            numRetries: 2,
            retryScheduleMs: [60, 120, 240],
        );
    }
}
