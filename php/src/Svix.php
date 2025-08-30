<?php

// this file is @generated
declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use Svix\Api\Application;
use Svix\Api\Authentication;
use Svix\Api\BackgroundTask;
use Svix\Api\Endpoint;
use Svix\Api\Environment;
use Svix\Api\EventType;
use Svix\Api\Health;
use Svix\Api\Ingest;
use Svix\Api\Integration;
use Svix\Api\Message;
use Svix\Api\MessageAttempt;
use Svix\Api\OperationalWebhook;
use Svix\Api\Statistics;
use Svix\Request\SvixHttpClient;

class Svix
{
    public Application $application;
    public Authentication $authentication;
    public BackgroundTask $backgroundTask;
    public Endpoint $endpoint;
    public Environment $environment;
    public EventType $eventType;
    public Health $health;
    public Ingest $ingest;
    public Integration $integration;
    public Message $message;
    public MessageAttempt $messageAttempt;
    public OperationalWebhook $operationalWebhook;
    public Statistics $statistics;

    public function __construct(
        string $apiKey,
        ?SvixOptions $options = null,
        ?Client $httpClient = null,
    ) {
        $baseUrl = $options?->serverUrl ?? Utils::getServerUrlFromToken($apiKey);

        $svixHttpClient = new SvixHttpClient(
            token: $apiKey,
            baseUrl: $baseUrl,
            guzzleClient: $httpClient ?? $this->createHttpClient(),
        );

        $this->application = new Application($svixHttpClient);
        $this->authentication = new Authentication($svixHttpClient);
        $this->backgroundTask = new BackgroundTask($svixHttpClient);
        $this->endpoint = new Endpoint($svixHttpClient);
        $this->environment = new Environment($svixHttpClient);
        $this->eventType = new EventType($svixHttpClient);
        $this->health = new Health($svixHttpClient);
        $this->ingest = new Ingest($svixHttpClient);
        $this->integration = new Integration($svixHttpClient);
        $this->message = new Message($svixHttpClient);
        $this->messageAttempt = new MessageAttempt($svixHttpClient);
        $this->operationalWebhook = new OperationalWebhook($svixHttpClient);
        $this->statistics = new Statistics($svixHttpClient);
    }

    /**
     * Create and configure the HTTP client with retry logic.
     */
    private function createHttpClient(): Client
    {
        $stack = HandlerStack::create();

        // Add retry middleware
        $stack->push(Middleware::retry(
            $this->retryDecider(),
            $this->retryDelay()
        ));

        $defaultOptions = [
            'handler' => $stack,
            'timeout' => 30,
        ];

        return new Client(array_merge($defaultOptions));
    }

    private function retryDecider(): callable
    {
        return function (
            int $retries,
            \Psr\Http\Message\RequestInterface $request,
            ?\Psr\Http\Message\ResponseInterface $response = null,
            ?\Exception $exception = null,
        ): bool {
            if ($retries >= 2) {
                return false;
            }
            if ($exception && !$response) {
                return true;
            }
            if ($response && $response->getStatusCode() >= 500) {
                return true;
            }

            return false;
        };
    }

    private function retryDelay(): callable
    {
        return function (int $retries): int {
            return (int) (50 * pow(2, $retries));
        };
    }
}
