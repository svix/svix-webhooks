<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\ExpungeAllContentsOut;
use Svix\Models\ListResponseMessageOut;
use Svix\Models\MessageIn;
use Svix\Models\MessageOut;
use Svix\Request\SvixHttpClient;

class Message
{
    public MessagePoller $poller;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->poller = new MessagePoller($client);
    }

    /**
     * List all of the application's messages.
     *
     * The `before` and `after` parameters let you filter all items created before or after a certain date. These can be
     * used alongside an iterator to paginate over results within a certain window.
     *
     * Note that by default this endpoint is limited to retrieving 90 days' worth of data
     * relative to now or, if an iterator is provided, 90 days before/after the time indicated
     * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
     * set the `before` or `after` parameter as appropriate.
     */
    public function list(
        string $appId,
        ?MessageListOptions $options = null,
    ): ListResponseMessageOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/msg");
        if (null !== $options) {
            $request->setQueryParam('limit', $options->limit);
            $request->setQueryParam('iterator', $options->iterator);
            $request->setQueryParam('channel', $options->channel);
            $request->setQueryParam('before', $options->before);
            $request->setQueryParam('after', $options->after);
            $request->setQueryParam('with_content', $options->withContent);
            $request->setQueryParam('tag', $options->tag);
            $request->setQueryParam('event_types', $options->eventTypes);
        }
        $res = $this->client->send($request);

        return ListResponseMessageOut::fromJson($res);
    }

    /**
     * Creates a new message and dispatches it to all of the application's endpoints.
     *
     * The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
     * If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
     *
     * The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
     * Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
     *
     * The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
     */
    public function create(
        string $appId,
        MessageIn $messageIn,
        ?MessageCreateOptions $options = null,
    ): MessageOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/msg");
        if (null !== $options) {
            $request->setQueryParam('with_content', $options->withContent);
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $request->setBody(json_encode($messageIn));
        $res = $this->client->send($request);

        return MessageOut::fromJson($res);
    }

    /**
     * Delete all message payloads for the application.
     *
     * This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan.
     *
     * A completed task will return a payload like the following:
     * ```json
     * {
     *   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
     *   "status": "finished",
     *   "task": "application.purge_content",
     *   "data": {
     *     "messagesPurged": 150
     *   }
     * }
     * ```
     */
    public function expungeAllContents(
        string $appId,
        ?MessageExpungeAllContentsOptions $options = null,
    ): ExpungeAllContentsOut {
        $request = $this->client->newReq('POST', "/api/v1/app/{$appId}/msg/expunge-all-contents");
        if (null !== $options) {
            $request->setHeaderParam('idempotency-key', $options->idempotencyKey);
        }
        $res = $this->client->send($request);

        return ExpungeAllContentsOut::fromJson($res);
    }

    /** Get a message by its ID or eventID. */
    public function get(
        string $appId,
        string $msgId,
        ?MessageGetOptions $options = null,
    ): MessageOut {
        $request = $this->client->newReq('GET', "/api/v1/app/{$appId}/msg/{$msgId}");
        if (null !== $options) {
            $request->setQueryParam('with_content', $options->withContent);
        }
        $res = $this->client->send($request);

        return MessageOut::fromJson($res);
    }

    /**
     * Delete the given message's payload.
     *
     * Useful in cases when a message was accidentally sent with sensitive content.
     * The message can't be replayed or resent once its payload has been deleted or expired.
     */
    public function expungeContent(
        string $appId,
        string $msgId,
    ): void {
        $request = $this->client->newReq('DELETE', "/api/v1/app/{$appId}/msg/{$msgId}/content");
        $res = $this->client->sendNoResponseBody($request);
    }
}
