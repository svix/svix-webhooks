<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class McpEventsUnsubscribeIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string     $name      the event the subscription was made for
     * @param array|null $arguments The filter arguments the subscription was made with, as given to
     *                              `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
     *                              in OpenAI's MCP Events docs.
     */
    private function __construct(
        public readonly string $name,
        public readonly McpEventsUnsubscribeDeliveryIn $delivery,
        public readonly ?array $arguments = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of McpEventsUnsubscribeIn with required fields.
     */
    public static function create(
        string $name,
        McpEventsUnsubscribeDeliveryIn $delivery,
    ): self {
        return new self(
            name: $name,
            arguments: null,
            delivery: $delivery,
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'McpEventsUnsubscribeIn'),
            arguments: \Svix\Utils::getValFromJson($data, 'arguments', false, 'McpEventsUnsubscribeIn'),
            delivery: \Svix\Utils::deserializeObject($data, 'delivery', true, 'McpEventsUnsubscribeIn', [McpEventsUnsubscribeDeliveryIn::class, 'fromMixed'])
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
