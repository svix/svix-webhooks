<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class McpEventsSubscribeDeliveryIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param McpEventsDeliveryMode $mode   How events are delivered. Only `webhook` is supported.
     * @param string                $url    the callback URL events are sent to
     * @param string                $secret The secret events are signed with. MCP Events requires a `whsec_`
     *                                      secret.
     */
    private function __construct(
        public readonly McpEventsDeliveryMode $mode,
        public readonly string $url,
        public readonly string $secret,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of McpEventsSubscribeDeliveryIn with required fields.
     */
    public static function create(
        McpEventsDeliveryMode $mode,
        string $url,
        string $secret,
    ): self {
        return new self(
            mode: $mode,
            url: $url,
            secret: $secret,
            setFields: ['mode' => true, 'url' => true, 'secret' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'mode' => $this->mode,
            'url' => $this->url,
            'secret' => $this->secret];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            mode: \Svix\Utils::deserializeObject($data, 'mode', true, 'McpEventsSubscribeDeliveryIn', [McpEventsDeliveryMode::class, 'fromMixed']),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'McpEventsSubscribeDeliveryIn'),
            secret: \Svix\Utils::deserializeString($data, 'secret', true, 'McpEventsSubscribeDeliveryIn')
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
