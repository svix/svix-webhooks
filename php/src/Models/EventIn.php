<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $eventType The event type's name
     */
    private function __construct(
        public readonly string $eventType,
        public readonly string $payload,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventIn with required fields.
     */
    public static function create(
        string $eventType,
        string $payload,
    ): self {
        return new self(
            eventType: $eventType,
            payload: $payload,
            setFields: ['eventType' => true, 'payload' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventType' => $this->eventType,
            'payload' => $this->payload];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'EventIn'),
            payload: \Svix\Utils::deserializeString($data, 'payload', true, 'EventIn')
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
