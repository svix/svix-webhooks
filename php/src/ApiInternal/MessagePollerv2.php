<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\ApiInternal;

use Svix\Exception\ApiException;
use Svix\Models\PollerV2CommitIn;
use Svix\Models\PollerV2PollOut;
use Svix\Request\SvixHttpClient;

class MessagePollerv2
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Poll messages from a sink.
     *
     * @throws ApiException
     */
    public function consumerPoll(
        string $appId,
        string $sinkId,
        string $consumerId,
        MessagePollerv2ConsumerPollOptions $options = new MessagePollerv2ConsumerPollOptions(),
    ): PollerV2PollOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/polling-endpoint/{$sinkId}/consumer/{$consumerId}");
        $request->setQueryParam('limit', $options->limit);
        $request->setQueryParam('lease_duration_ms', $options->leaseDurationMs);
        $request->setQueryParam('starting_position', $options->startingPosition);
        $res = $this->client->send($request);

        return PollerV2PollOut::fromJson($res);
    }

    /**
     * Ack a message offset for a sink's consumer.
     *
     * @throws ApiException
     */
    public function consumerCommit(
        string $appId,
        string $sinkId,
        string $consumerId,
        PollerV2CommitIn $pollerV2CommitIn,
        MessagePollerv2ConsumerCommitOptions $options = new MessagePollerv2ConsumerCommitOptions(),
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/polling-endpoint/{$sinkId}/consumer/{$consumerId}/commit");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($pollerV2CommitIn));
        $res = $this->client->sendNoResponseBody($request);
    }
}
