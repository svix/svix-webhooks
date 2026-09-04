<?php

declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use Svix\ApiInternal\EndpointAutoConfigDeprecated;
use Svix\ApiInternal\DestinationAutoconfig;
use Svix\ApiInternal\MessagePollerv2;
use Svix\Exception\ApiException;
use Svix\Models\AutoConfigSinkType;
use Svix\Models\DestinationIn;
use Svix\Models\DestinationOut;
use Svix\Models\EndpointOut;
use Svix\Models\PollerV2CommitIn;
use Svix\Models\PollerV2PollOut;
use Svix\Models\SinkInCommon;
use Svix\Models\SinkStatus;
use Svix\Models\SubscribeIn;
use Svix\Request\SvixHttpClient;

final class AutoConfigConsumer
{
    private string $appId;

    private ?string $sinkId = null;

    private ?string $autoconfigId = null;

    private SinkInCommon $sinkIn;

    private SvixHttpClient $client;

    /**
     * @throws \InvalidArgumentException if the token is invalid
     */
    public function __construct(string $token, SinkInCommon $sinkIn)
    {
        $content = AutoConfig::decodeToken($token);
        $this->appId = $content['app_id'];
        $this->sinkId = $content['endpoint_id'] ?? null;
        $this->autoconfigId = $content['autoconfig_id'] ?? null;
        $this->sinkIn = $sinkIn;

        $opts = SvixOptions::newDefault($content['token_plaintext']);
        $opts->serverUrl = $content['server_url'];

        $this->client = new SvixHttpClient(
            baseUrl: $content['server_url'],
            token: $content['token_plaintext'],
            guzzleClient: new Client(),
            opts: $opts,
        );
    }

    /**
     * @throws ApiException
     */
    public function subscribe(): DestinationOut
    {
        if ($this->autoconfigId !== null) {
            $destination = (new DestinationAutoconfig($this->client))->subscribe(
                $this->appId,
                $this->autoconfigId,
                self::sinkInCommonToPollingDestination($this->sinkIn),
            );
            $this->sinkId = $destination->id;

            return $destination;
        }

        $endpoint = (new EndpointAutoConfigDeprecated($this->client))->update(
            $this->appId,
            $this->sinkId ?? '',
            SubscribeIn::create()->withSink(
                AutoConfigSinkType::create('poller', $this->sinkIn),
            ),
        );

        return self::destinationOutFromV1Endpoint($endpoint);
    }

    /**
     * @throws ApiException
     */
    public function receive(
        string $consumerId,
        ?MessagePollerv2\MessagePollerv2ConsumerPollOptions $options = null,
    ): PollerV2PollOut {
        $this->sinkId = $this->sinkId ?? $this->subscribe()->id;

        return (new MessagePollerv2($this->client))->consumerPoll(
            $this->appId,
            $this->sinkId,
            $consumerId,
            $options ?? new MessagePollerv2\MessagePollerv2ConsumerPollOptions(),
        );
    }

    /**
     * @throws ApiException
     */
    public function commit(
        string $consumerId,
        int $offset,
        ?MessagePollerv2\MessagePollerv2ConsumerCommitOptions $options = null,
    ): void {
        $this->sinkId = $this->sinkId ?? $this->subscribe()->id;

        (new MessagePollerv2($this->client))->consumerCommit(
            $this->appId,
            $this->sinkId,
            $consumerId,
            PollerV2CommitIn::create($offset),
            $options ?? new MessagePollerv2\MessagePollerv2ConsumerCommitOptions(),
        );
    }

    private static function destinationOutFromV1Endpoint(EndpointOut $endpoint): DestinationOut
    {
        return DestinationOut::fromMixed([
            'id' => $endpoint->id,
            'uid' => $endpoint->uid,
            'status' => $endpoint->disabled
                ? SinkStatus::DISABLED->value
                : SinkStatus::ENABLED->value,
            'currentIterator' => '',
            'createdAt' => $endpoint->createdAt->format('c'),
            'updatedAt' => $endpoint->updatedAt->format('c'),
            'batchSize' => 0,
            'maxWaitSecs' => 0,
            'eventTypes' => $endpoint->eventTypes,
            'channels' => $endpoint->channels,
            'metadata' => $endpoint->metadata,
            'type' => 'pollingEndpoint',
            'config' => new \stdClass(),
        ]);
    }

    private static function sinkInCommonToPollingDestination(SinkInCommon $sink): DestinationIn
    {
        return DestinationIn::fromMixed([
            'type' => 'pollingEndpoint',
            'uid' => $sink->uid,
            'eventTypes' => $sink->eventTypes,
            'channels' => $sink->channels,
            'metadata' => $sink->metadata,
        ]);
    }
}
