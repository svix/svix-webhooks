<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PollingEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<PollingEndpointMessageOut> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly bool $done,
        public readonly string $iterator,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollingEndpointOut with required fields.
     */
    public static function create(
        array $data,
        bool $done,
        string $iterator,
    ): self {
        return new self(
            data: $data,
            done: $done,
            iterator: $iterator,
            setFields: ['data' => true, 'done' => true, 'iterator' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['data' => $this->data,
            'done' => $this->done,
            'iterator' => $this->iterator];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'PollingEndpointOut', [PollingEndpointMessageOut::class, 'fromMixed']),
            done: \Svix\Utils::deserializeBool($data, 'done', true, 'PollingEndpointOut'),
            iterator: \Svix\Utils::deserializeString($data, 'iterator', true, 'PollingEndpointOut')
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
