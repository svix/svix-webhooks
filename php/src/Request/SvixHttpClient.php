<?php

declare(strict_types=1);

namespace Svix\Request;

use GuzzleHttp\Exception\RequestException;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Svix\Exception\ApiException;
use Svix\SvixOptions;
use Svix\Utils;
use Svix\Version;

class SvixHttpClient
{
    /** @var int[] Retry schedule in milliseconds - defines sleep time before each retry attempt */
    private array $retryScheduleMs = [50, 100];

    public function __construct(
        private string $baseUrl,
        private string $token,
        private \GuzzleHttp\Client $guzzleClient,
        private SvixOptions $opts
    ) {}

    public function newReq(
        string $method,
        string $path
    ): SvixRequest {
        $url = $this->baseUrl . $path;
        return new SvixRequest($method, $url);
    }

    public function send(SvixRequest $req): ?string
    {
        $response = $this->sendInner($req);
        return $response['body'];
    }

    public function sendNoResponseBody(SvixRequest $req): void
    {
        $this->sendInner($req);
    }


    private function sendInner(SvixRequest $req): array
    {
        // Set idempotency key for POST requests
        if (!isset($req->headerParams['idempotency-key']) && $req->method === 'POST') {
            $req->headerParams['idempotency-key'] = 'auto_' . Utils::uuid4();
        }

        $maxRetries = count($this->retryScheduleMs);
        $attempt = 0;
        $lastException = null;

        while ($attempt <= $maxRetries) {

            // Build headers
            $headers = array_merge([
                'Authorization' => 'Bearer ' . $this->token,
                'User-Agent' => 'svix-libs/' . Version::VERSION . '/php',
                'svix-req-id' => (string)random_int(0, PHP_INT_MAX),
            ], $req->headerParams);

            if ($req->body !== null) {
                $headers['Content-Type'] = 'application/json';
            }

            $query = '';
            if (!empty($req->queryParams)) {
                $query .= '?' . \http_build_query($req->queryParams, '', '&', \PHP_QUERY_RFC3986);
            }
            $uri = $req->url . $query;

            $request = new \GuzzleHttp\Psr7\Request($req->method, $uri, $headers, $req->body, '1.1');

            $timeoutSeconds = $this->opts->timeoutMs !== null ? $this->opts->timeoutMs / 1000.0 : 30.0;
            $options = ["timeout" => $timeoutSeconds];

            try {
                $response = $this->guzzleClient->send($request, $options);
                if ($this->opts->debug) {
                    logHttpRequestResponse($request, $response);
                }

                $statusCode = $response->getStatusCode();
                $body = $response->getBody()->getContents();
                $responseHeaders = $this->formatHeaders($response->getHeaders());

                $result = [
                    'status' => $statusCode,
                    'body' => $body,
                    'headers' => $responseHeaders
                ];

                // Check if we should retry based on status code
                if ($statusCode >= 500 && $attempt < $maxRetries) {
                    $this->sleepFromSchedule($attempt);
                    $attempt++;
                    continue;
                }

                return $this->filterResponseForErrors($result);
            } catch (RequestException $e) {
                $response = $e->getResponse();
                if ($this->opts->debug) {
                    logHttpRequestResponse($request, $response);
                }
                if ($response) {
                    $statusCode = $response->getStatusCode();
                    $body = $response->getBody()->getContents();
                    $responseHeaders = $this->formatHeaders($response->getHeaders());

                    $result = [
                        'status' => $statusCode,
                        'body' => $body,
                        'headers' => $responseHeaders
                    ];

                    // Check if we should retry based on status code
                    if ($statusCode >= 500 && $attempt < $maxRetries) {
                        $this->sleepFromSchedule($attempt);
                        $attempt++;
                        continue;
                    }

                    return $this->filterResponseForErrors($result);
                }

                // For connection errors, retry if we haven't exceeded max attempts
                if ($attempt < $maxRetries) {
                    $lastException = $e;
                    $this->sleepFromSchedule($attempt);
                    $attempt++;
                    continue;
                }

                throw new ApiException(0, $e->getMessage(), []);
            }
        }

        // If we get here, all retries were exhausted due to connection errors
        if ($lastException) {
            throw new ApiException(0, $lastException->getMessage(), []);
        }

        // This should never be reached, but just in case
        throw new ApiException(0, 'Max retries exceeded', []);
    }

    /**
     * Sleep based on the retry schedule
     */
    private function sleepFromSchedule(int $attemptIndex): void
    {
        if (isset($this->retryScheduleMs[$attemptIndex])) {
            $delayMs = $this->retryScheduleMs[$attemptIndex];
            usleep($delayMs * 1000); // usleep takes microseconds
        }
    }

    private function formatHeaders(array $headers): array
    {
        $formatted = [];
        foreach ($headers as $name => $values) {
            $formatted[strtolower($name)] = is_array($values) ? implode(', ', $values) : $values;
        }
        return $formatted;
    }


    /**
     * @param array{status: int, body: string, headers: array} $response
     * @return array{status: int, body: string, headers: array}
     * @throws ApiException
     */
    private function filterResponseForErrors(array $response): array
    {
        if ($response['status'] < 300) {
            return $response;
        }

        // Decode JSON body for error responses
        $errorBody = null;
        if (!empty($response['body'])) {
            try {
                $errorBody = json_decode($response['body'], true, 512, JSON_THROW_ON_ERROR);
            } catch (\JsonException $e) {
                // If JSON decode fails, use raw body
                $errorBody = $response['body'];
            }
        }

        throw new ApiException(
            $response['status'],
            $errorBody ?? $response['body'],
            $response['headers']
        );
    }
}



function dumpHttpRequest(RequestInterface $request, bool $body = true): string
{
    $dump = sprintf("%s %s HTTP/1.1\r\n", $request->getMethod(), $request->getRequestTarget());

    // Add Host header first if present
    if ($request->hasHeader('Host')) {
        $dump .= sprintf("Host: %s\r\n", $request->getHeaderLine('Host'));
    }

    // Add all other headers - each on its own line
    foreach ($request->getHeaders() as $name => $values) {
        if (strtolower($name) !== 'host') {
            foreach ($values as $value) {
                $dump .= sprintf("%s: %s\r\n", $name, $value);
            }
        }
    }

    $dump .= "\r\n";

    if ($body) {
        $requestBody = $request->getBody();
        if ($requestBody->getSize() > 0) {
            if ($requestBody->isSeekable()) {
                $requestBody->rewind();
            }
            $dump .= $requestBody->getContents();
            if ($requestBody->isSeekable()) {
                $requestBody->rewind();
            }
        }
    }

    return $dump;
}

function dumpHttpResponse(ResponseInterface $response, bool $body = true): string
{
    $dump = sprintf("HTTP/1.1 %d %s\r\n", $response->getStatusCode(), $response->getReasonPhrase());

    // Each header on its own line
    foreach ($response->getHeaders() as $name => $values) {
        foreach ($values as $value) {
            $dump .= sprintf("%s: %s\r\n", $name, $value);
        }
    }

    $dump .= "\r\n";

    if ($body) {
        $responseBody = $response->getBody();
        if ($responseBody->getSize() > 0) {
            if ($responseBody->isSeekable()) {
                $responseBody->rewind();
            }
            $dump .= $responseBody->getContents();
            if ($responseBody->isSeekable()) {
                $responseBody->rewind();
            }
        }
    }

    return $dump;
}

function logHttpRequestResponse(RequestInterface $request, ResponseInterface $response)
{
    if (getenv("SVIX_DEBUG")) {
        $output = dumpHttpRequest($request) . "\n";

        if ($response) {
            $output .= dumpHttpResponse($response) . "\n";
        }

        $output .= str_repeat('-', 50) . "\n\n";
        echo $output . "\n";
    }
}
