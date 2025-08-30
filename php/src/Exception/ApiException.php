<?php
declare(strict_types=1);

namespace Svix\Exception;

class ApiException extends \Exception
{
    /** @var array<string, string> */
    public array $headers = [];

    /**
     * @param int $status_code HTTP status code
     * @param mixed $body Response body (can be array, object, string, etc.)
     * @param array<string, string> $headers Response headers
     */
    public function __construct(
        public readonly int $status_code,
        public readonly mixed $body,
        array $headers = []
    ) {
        // Build the exception message
        $message = sprintf(
            "HTTP-Code: %d\nHeaders: %s",
            $status_code,
            json_encode($headers)
        );

        parent::__construct($message, $status_code);

        // Store headers
        $this->headers = $headers;
    }

    /**
     * Get the response body
     *
     * @return mixed
     */
    public function getBody(): mixed
    {
        return $this->body;
    }

    /**
     * Get the HTTP status code
     *
     * @return int
     */
    public function getStatusCode(): int
    {
        return $this->code;
    }

    /**
     * Get response headers
     *
     * @return array<string, string>
     */
    public function getHeaders(): array
    {
        return $this->headers;
    }

    /**
     * Get a specific header value
     *
     * @param string $name
     * @return string|null
     */
    public function getHeader(string $name): ?string
    {
        return $this->headers[$name] ?? null;
    }
}
