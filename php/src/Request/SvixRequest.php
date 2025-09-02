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
}
