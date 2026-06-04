<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RabbitMqPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $routingKey = null,
        public readonly ?string $uri = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RabbitMqPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            routingKey: null,
            uri: null,
            setFields: []
        );
    }

    public function withRoutingKey(?string $routingKey): self
    {
        $setFields = $this->setFields;
        $setFields['routingKey'] = true;

        return new self(
            routingKey: $routingKey,
            uri: $this->uri,
            setFields: $setFields
        );
    }

    public function withUri(?string $uri): self
    {
        $setFields = $this->setFields;
        $setFields['uri'] = true;

        return new self(
            routingKey: $this->routingKey,
            uri: $uri,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->routingKey) {
            $data['routingKey'] = $this->routingKey;
        }
        if (null !== $this->uri) {
            $data['uri'] = $this->uri;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            routingKey: \Svix\Utils::deserializeString($data, 'routingKey', false, 'RabbitMqPatchConfig'),
            uri: \Svix\Utils::deserializeString($data, 'uri', false, 'RabbitMqPatchConfig')
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
