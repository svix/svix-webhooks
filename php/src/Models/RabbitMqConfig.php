<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a RabbitMq sink. */
class RabbitMqConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $uri,
        public readonly string $routingKey,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RabbitMqConfig with required fields.
     */
    public static function create(
        string $uri,
        string $routingKey,
    ): self {
        return new self(
            uri: $uri,
            routingKey: $routingKey,
            setFields: ['uri' => true, 'routingKey' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'uri' => $this->uri,
            'routingKey' => $this->routingKey];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            uri: \Svix\Utils::deserializeString($data, 'uri', true, 'RabbitMqConfig'),
            routingKey: \Svix\Utils::deserializeString($data, 'routingKey', true, 'RabbitMqConfig')
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
