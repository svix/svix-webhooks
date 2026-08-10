<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RabbitMqConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $routingKey,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RabbitMqConfigOut with required fields.
     */
    public static function create(
        string $routingKey,
    ): self {
        return new self(
            routingKey: $routingKey,
            setFields: ['routingKey' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'routingKey' => $this->routingKey];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            routingKey: \Svix\Utils::deserializeString($data, 'routingKey', true, 'RabbitMqConfigOut')
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
