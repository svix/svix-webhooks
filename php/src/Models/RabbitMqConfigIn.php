<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a RabbitMq sink. */
class RabbitMqConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $uri URI to connect to
     *
     * Note that the VHost must be percent-escaped, so a default URI would look
     * like `amqp://user:pass@host/%2F`
     * @param string    $routingKey Routing key for message dispatch
     * @param bool|null $mandatory  If true, then dispatches will fail if there is no attached queue; if false, they are
     *                              silently dropped (this was previously the default)
     */
    private function __construct(
        public readonly string $uri,
        public readonly string $routingKey,
        public readonly ?bool $mandatory = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RabbitMqConfigIn with required fields.
     */
    public static function create(
        string $uri,
        string $routingKey,
    ): self {
        return new self(
            uri: $uri,
            routingKey: $routingKey,
            mandatory: null,
            setFields: ['uri' => true, 'routingKey' => true]
        );
    }

    public function withMandatory(?bool $mandatory): self
    {
        $setFields = $this->setFields;
        $setFields['mandatory'] = true;

        return new self(
            uri: $this->uri,
            routingKey: $this->routingKey,
            mandatory: $mandatory,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'uri' => $this->uri,
            'routingKey' => $this->routingKey];

        if (null !== $this->mandatory) {
            $data['mandatory'] = $this->mandatory;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            uri: \Svix\Utils::deserializeString($data, 'uri', true, 'RabbitMqConfigIn'),
            routingKey: \Svix\Utils::deserializeString($data, 'routingKey', true, 'RabbitMqConfigIn'),
            mandatory: \Svix\Utils::deserializeBool($data, 'mandatory', false, 'RabbitMqConfigIn')
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
