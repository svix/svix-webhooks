<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a RabbitMq sink. */
class RabbitMqConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $routingKey,
        public readonly string $uri,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RabbitMqConfig with required fields.
     */
    public static function create(
        string $routingKey,
        string $uri,
    ): self {
        return new self(
            routingKey: $routingKey,
            uri: $uri,
            setFields: ['routingKey' => true, 'uri' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'routingKey' => $this->routingKey,
            'uri' => $this->uri];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            routingKey: \Svix\Utils::deserializeString($data, 'routingKey', true, 'RabbitMqConfig'),
            uri: \Svix\Utils::deserializeString($data, 'uri', true, 'RabbitMqConfig')
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
