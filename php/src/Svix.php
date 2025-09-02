<?php

// this file is @generated
declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
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
        string $token,
        ?SvixOptions $options = null,
        ?Client $httpClient = null,
    ) {
        $baseUrl = $options?->serverUrl ?? Utils::getServerUrlFromToken($token);

        $svixHttpClient = new SvixHttpClient(
            token: $token,
            baseUrl: $baseUrl,
            guzzleClient: $httpClient ?? new Client(),
            opts: $options ?? SvixOptions::newDefault($token),
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
}
