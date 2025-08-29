<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PollingEndpointConsumerSeekOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $iterator,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollingEndpointConsumerSeekOut with required fields.
     */
    public static function create(
        string $iterator,
    ): self {
        return new self(
            iterator: $iterator,
            setFields: ['iterator' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['iterator' => $this->iterator];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            iterator: \Svix\Utils::deserializeString($data, 'iterator', true, 'PollingEndpointConsumerSeekOut')
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
