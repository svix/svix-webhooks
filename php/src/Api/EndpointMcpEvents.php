<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Exception\ApiException;
use Svix\Models\McpEventsSubscribeIn;
use Svix\Models\McpEventsSubscribeOut;
use Svix\Models\McpEventsUnsubscribeIn;
use Svix\Request\SvixHttpClient;

class EndpointMcpEvents
{
    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
    }

    /**
     * Create or refresh an MCP Events subscription.
     *
     * @throws ApiException
     */
    public function subscribe(
        string $appId,
        McpEventsSubscribeIn $mcpEventsSubscribeIn,
        EndpointMcpEventsSubscribeOptions $options = new EndpointMcpEventsSubscribeOptions(),
    ): McpEventsSubscribeOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/mcp-events/subscribe");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($mcpEventsSubscribeIn));
        $res = $this->client->send($request);

        return McpEventsSubscribeOut::fromJson($res);
    }

    /**
     * Stop an MCP Events subscription.
     *
     * Forward an MCP server's `events/unsubscribe` requests here. The
     * subscription's endpoint is deleted. Unsubscribing from a subscription that
     * doesn't exist succeeds, so this can be repeated.
     *
     * @throws ApiException
     */
    public function unsubscribe(
        string $appId,
        McpEventsUnsubscribeIn $mcpEventsUnsubscribeIn,
        EndpointMcpEventsUnsubscribeOptions $options = new EndpointMcpEventsUnsubscribeOptions(),
    ): void {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/mcp-events/unsubscribe");
        $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        $request->setBody(json_encode($mcpEventsUnsubscribeIn));
        $res = $this->client->sendNoResponseBody($request);
    }
}
