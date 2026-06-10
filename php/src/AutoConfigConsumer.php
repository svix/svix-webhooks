<?php

declare(strict_types=1);

namespace Svix;

use GuzzleHttp\Client;
use Svix\ApiInternal\MessagePollerv2;
use Svix\Models\EndpointOut;
use Svix\Models\PollerV2CommitIn;
use Svix\Models\PollerV2PollOut;
use Svix\Models\SinkInCommon;
use Svix\Request\SvixHttpClient;

final class AutoConfigConsumer
{
    private string $appId;

    private string $sinkId;

    private SinkInCommon $sinkIn;

    private SvixHttpClient $client;

    public function __construct(string $token, SinkInCommon $sinkIn)
    {
        $content = AutoConfig::decodeToken($token);
        $this->appId = $content['app_id'];
        $this->sinkId = $content['endpoint_id'];
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

    public function subscribe(): EndpointOut
    {
        $body = json_encode([
            'sink' => [
                'type' => 'poller',
                'config' => $this->sinkIn,
            ],
        ], JSON_THROW_ON_ERROR);

        $request = $this->client->newReq(
            'PUT',
            "/api/v1/app/{$this->appId}/endpoint/{$this->sinkId}/auto-config",
        );
        $request->setBody($body);
        $res = $this->client->send($request);

        return EndpointOut::fromJson($res);
    }

    public function receive(
        string $consumerId,
        ?MessagePollerv2\MessagePollerv2ConsumerPollOptions $options = null,
    ): PollerV2PollOut {
        return (new MessagePollerv2($this->client))->consumerPoll(
            $this->appId,
            $this->sinkId,
            $consumerId,
            $options,
        );
    }

    public function commit(
        string $consumerId,
        int $offset,
        ?MessagePollerv2\MessagePollerv2ConsumerCommitOptions $options = null,
    ): void {
        (new MessagePollerv2($this->client))->consumerCommit(
            $this->appId,
            $this->sinkId,
            $consumerId,
            PollerV2CommitIn::create($offset),
            $options,
        );
    }
}
