<?php

declare(strict_types=1);

namespace Svix\Request;

use GuzzleHttp\Client;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use GuzzleHttp\Psr7\Request;
use GuzzleHttp\Psr7\Response;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Svix\Exception\ApiException;
use Svix\Request\SvixRequestContext;


enum HttpMethod: string
{
    case GET = 'GET';
    case HEAD = 'HEAD';
    case POST = 'POST';
    case PUT = 'PUT';
    case DELETE = 'DELETE';
    case CONNECT = 'CONNECT';
    case OPTIONS = 'OPTIONS';
    case TRACE = 'TRACE';
    case PATCH = 'PATCH';
}

class SvixRequest
{
    public string $method;
    public ?string $body = null;
    public string $url;
    public array $queryParams = [];
    public array $headerParams = [];

    public function __construct(string $method, string $url)
    {
        $this->method = $method;
        $this->url = $url;
    }

    public function setQueryParam(string $name, $value): void
    {
        if ($value === null) {
            return;
        }

        if (is_string($value)) {
            $this->queryParams[$name] = $value;
        } elseif (is_bool($value)) {
            $this->queryParams[$name] = $value ? 'true' : 'false';
        } elseif (is_numeric($value)) {
            $this->queryParams[$name] = (string)$value;
        } elseif ($value instanceof \DateTimeImmutable) {
            $this->queryParams[$name] = $value->format('c');
        } elseif ($value instanceof \BackedEnum) {
            $this->queryParams[$name] = (string)$value->value;
        } elseif (is_array($value)) {
            if (count($value) > 0) {
                $this->queryParams[$name] = implode(',', $value);
            }
        } else {
            throw new \Exception("Query parameter {$name} has unsupported type");
        }
    }

    public function setHeaderParam(string $name, ?string $value): void
    {
        if ($value === null) {
            return;
        }
        $this->headerParams[$name] = $value;
    }

    public function setBody($value): void
    {
        $this->body = $value;
    }

    // private function createGuzzleClient(): Client
    // {
    //     $stack = HandlerStack::create();

    //     // Add retry middleware
    //     $stack->push(Middleware::retry(
    //         $this->retryDecider(),
    //         $this->retryDelay()
    //     ));

    //     // Add retry count header middleware
    //     $stack->push(Middleware::mapRequest(function (RequestInterface $request) {
    //         static $retryCount = 0;

    //         if ($request->hasHeader('svix-retry-count')) {
    //             $retryCount++;
    //         } else {
    //             $retryCount = 0;
    //         }

    //         if ($retryCount > 0) {
    //             return $request->withHeader('svix-retry-count', (string)$retryCount);
    //         }

    //         return $request;
    //     }));

    //     return new Client(['handler' => $stack]);
    // }

    // /**
    //  * Retry decider function for Guzzle middleware
    //  */
    // private function retryDecider(): callable
    // {
    //     return function (
    //         int $retries,
    //         RequestInterface $request,
    //         ?ResponseInterface $response = null,
    //         ?\Exception $exception = null
    //     ): bool {
    //         // Retry up to 2 times (3 total attempts)
    //         if ($retries >= 2) {
    //             return false;
    //         }

    //         // Retry on connection errors
    //         if ($exception && !$response) {
    //             return true;
    //         }

    //         // Retry on 5xx status codes
    //         if ($response && $response->getStatusCode() >= 500) {
    //             return true;
    //         }

    //         return false;
    //     };
    // }

    // /**
    //  * Retry delay function for Guzzle middleware
    //  */
    // private function retryDelay(): callable
    // {
    //     return function (int $retries): int {
    //         // Exponential backoff: 50ms, 100ms, 200ms
    //         return (int)(50 * pow(2, $retries));
    //     };
    // }



    // private static function uuid4(): string
    // {
    //     $data = random_bytes(16);

    //     $data[6] = chr(ord($data[6]) & 0x0f | 0x40); // set version to 0100
    //     $data[8] = chr(ord($data[8]) & 0x3f | 0x80); // set bits 6-7 to 10

    //     return vsprintf('%s%s-%s-%s-%s-%s%s%s', str_split(bin2hex($data), 4));
    // }
}
