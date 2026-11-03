<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class McpEventsSubscribeIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string     $name      The event to subscribe to. It must be an existing event type.
     * @param array|null $arguments The filter arguments of the `events/subscribe` request, as described by
     *                              the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
     *                              Together with `name` and the callback URL they identify the subscription
     *
     * They're stored in the endpoint's `mcpEventArguments` metadata, but
     * aren't used to filter messages; use `channels` for that.
     *
     * See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
     * in OpenAI's MCP Events docs.
     * @param int|null $ttlMs                    The requested lifetime of the subscription, in milliseconds. Omit for
     *                                           the default of 7 days, or set to `null` for a subscription that never
     *                                           expires.
     * @param int|null $secretGracePeriodSeconds How long the previous secret keeps signing when a refresh changes it,
     *                                           in seconds.
     *
     * Valid values are between 0 (immediate expiry) and 7 days. The default
     * is 5 minutes.
     * @param EndpointChallengeMode|null $challengeMode how the callback must respond to the verification challenge
     * @param list<string>|null          $channels      Only send messages with one of these channels to the subscription,
     *                                                  for example ones derived from `arguments`. Omit to send all messages
     *                                                  of the event type.
     */
    private function __construct(
        public readonly string $name,
        public readonly McpEventsSubscribeDeliveryIn $delivery,
        public readonly ?array $arguments = null,
        public readonly ?int $ttlMs = null,
        public readonly ?int $secretGracePeriodSeconds = null,
        public readonly ?EndpointChallengeMode $challengeMode = null,
        public readonly ?array $channels = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of McpEventsSubscribeIn with required fields.
     */
    public static function create(
        string $name,
        McpEventsSubscribeDeliveryIn $delivery,
    ): self {
        return new self(
            name: $name,
            arguments: null,
            delivery: $delivery,
            ttlMs: null,
            secretGracePeriodSeconds: null,
            challengeMode: null,
            channels: null,
            setFields: ['name' => true, 'delivery' => true]
        );
    }

    public function withArguments(?array $arguments): self
    {
        $setFields = $this->setFields;
        $setFields['arguments'] = true;

        return new self(
            name: $this->name,
            arguments: $arguments,
            delivery: $this->delivery,
            ttlMs: $this->ttlMs,
            secretGracePeriodSeconds: $this->secretGracePeriodSeconds,
            challengeMode: $this->challengeMode,
            channels: $this->channels,
            setFields: $setFields
        );
    }

    public function withTtlMs(?int $ttlMs): self
    {
        $setFields = $this->setFields;
        $setFields['ttlMs'] = true;

        return new self(
            name: $this->name,
            arguments: $this->arguments,
            delivery: $this->delivery,
            ttlMs: $ttlMs,
            secretGracePeriodSeconds: $this->secretGracePeriodSeconds,
            challengeMode: $this->challengeMode,
            channels: $this->channels,
            setFields: $setFields
        );
    }

    public function withSecretGracePeriodSeconds(?int $secretGracePeriodSeconds): self
    {
        $setFields = $this->setFields;
        $setFields['secretGracePeriodSeconds'] = true;

        return new self(
            name: $this->name,
            arguments: $this->arguments,
            delivery: $this->delivery,
            ttlMs: $this->ttlMs,
            secretGracePeriodSeconds: $secretGracePeriodSeconds,
            challengeMode: $this->challengeMode,
            channels: $this->channels,
            setFields: $setFields
        );
    }

    public function withChallengeMode(?EndpointChallengeMode $challengeMode): self
    {
        $setFields = $this->setFields;
        $setFields['challengeMode'] = true;

        return new self(
            name: $this->name,
            arguments: $this->arguments,
            delivery: $this->delivery,
            ttlMs: $this->ttlMs,
            secretGracePeriodSeconds: $this->secretGracePeriodSeconds,
            challengeMode: $challengeMode,
            channels: $this->channels,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            name: $this->name,
            arguments: $this->arguments,
            delivery: $this->delivery,
            ttlMs: $this->ttlMs,
            secretGracePeriodSeconds: $this->secretGracePeriodSeconds,
            challengeMode: $this->challengeMode,
            channels: $channels,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'delivery' => $this->delivery];

        if (null !== $this->arguments) {
            $data['arguments'] = $this->arguments;
        }
        if (isset($this->setFields['ttlMs'])) {
            $data['ttlMs'] = $this->ttlMs;
        }
        if (isset($this->setFields['secretGracePeriodSeconds'])) {
            $data['secretGracePeriodSeconds'] = $this->secretGracePeriodSeconds;
        }
        if (null !== $this->challengeMode) {
            $data['challengeMode'] = $this->challengeMode;
        }
        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'McpEventsSubscribeIn'),
            arguments: \Svix\Utils::getValFromJson($data, 'arguments', false, 'McpEventsSubscribeIn'),
            delivery: \Svix\Utils::deserializeObject($data, 'delivery', true, 'McpEventsSubscribeIn', [McpEventsSubscribeDeliveryIn::class, 'fromMixed']),
            ttlMs: \Svix\Utils::deserializeInt($data, 'ttlMs', false, 'McpEventsSubscribeIn'),
            secretGracePeriodSeconds: \Svix\Utils::deserializeInt($data, 'secretGracePeriodSeconds', false, 'McpEventsSubscribeIn'),
            challengeMode: \Svix\Utils::deserializeObject($data, 'challengeMode', false, 'McpEventsSubscribeIn', [EndpointChallengeMode::class, 'fromMixed']),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'McpEventsSubscribeIn')
        );
    }

    /**
     * Create an instance from a json string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode(json: $json, associative: true, depth: 512, flags: JSON_THROW_ON_ERROR);

        return self::fromMixed($data);
    }
}
