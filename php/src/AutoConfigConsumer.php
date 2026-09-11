<?php

declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use Svix\ApiInternal\AutoconfigSubscription;
use Svix\ApiInternal\EndpointAutoConfigDeprecated;
use Svix\ApiInternal\MessagePollerv2;
use Svix\ApiInternal\MessagePollerv2ConsumerCommitOptions;
use Svix\ApiInternal\MessagePollerv2ConsumerPollOptions;
use Svix\Exception\ApiException;
use Svix\Models\AutoConfigSinkType;
use Svix\Models\DestinationIn;
use Svix\Models\DestinationInConfig;
use Svix\Models\DestinationOut;
use Svix\Models\DestinationOutConfig;
use Svix\Models\DestinationStatus;
use Svix\Models\EndpointOut;
use Svix\Models\PollerV2CommitIn;
use Svix\Models\PollerV2PollOut;
use Svix\Models\SinkInCommon;
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
            $destination = (new AutoconfigSubscription($this->client))->destination->subscribe(
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
            SubscribeIn::create()->withSink(AutoConfigSinkType::poller($this->sinkIn)),
        );

        return self::destinationOutFromV1Endpoint($endpoint);
    }

    /**
     * @throws ApiException
     */
    public function receive(
        string $consumerId,
        MessagePollerv2ConsumerPollOptions $options = new MessagePollerv2ConsumerPollOptions(),
    ): PollerV2PollOut {
        return (new MessagePollerv2($this->client))->consumerPoll(
            $this->appId,
            $this->getSinkId(),
            $consumerId,
            $options,
        );
    }

    /**
     * @throws ApiException
     */
    public function commit(
        string $consumerId,
        int $offset,
        MessagePollerv2ConsumerCommitOptions $options = new MessagePollerv2ConsumerCommitOptions(),
    ): void {
        (new MessagePollerv2($this->client))->consumerCommit(
            $this->appId,
            $this->getSinkId(),
            $consumerId,
            PollerV2CommitIn::create($offset),
            $options,
        );
    }

    /**
     * @throws ApiException
     */
    private function getSinkId(): string
    {
        if ($this->sinkId !== null) {
            return $this->sinkId;
        }

        $destId = (new AutoconfigSubscription($this->client))->get(
            $this->appId,
            $this->autoconfigId ?? '',
        )->destId;

        if ($destId === null || $destId === '') {
            throw new \RuntimeException('autoconfig subscription is pending. Have you called subscribe()?');
        }

        return $destId;
    }

    private static function sinkInCommonToPollingDestination(SinkInCommon $sink): DestinationIn
    {
        return DestinationIn::create(DestinationInConfig::pollingEndpoint())
            ->withUid($sink->uid)
            ->withEventTypes($sink->eventTypes)
            ->withChannels($sink->channels)
            ->withMetadata($sink->metadata);
    }

    private static function destinationOutFromV1Endpoint(EndpointOut $endpoint): DestinationOut
    {
        $out = DestinationOut::create(
            DestinationOutConfig::pollingEndpoint(),
            $endpoint->id,
            $endpoint->disabled === true ? DestinationStatus::DISABLED : DestinationStatus::ENABLED,
            '',
            $endpoint->createdAt,
            $endpoint->updatedAt,
            0,
            0,
            $endpoint->metadata,
        )->withUid($endpoint->uid);

        if ($endpoint->eventTypes !== null) {
            $out = $out->withEventTypes($endpoint->eventTypes);
        }
        if ($endpoint->channels !== null) {
            $out = $out->withChannels($endpoint->channels);
        }

        return $out;
    }
}
