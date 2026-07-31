<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventStreamOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<EventOut> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly string $iterator,
        public readonly bool $done,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventStreamOut with required fields.
     */
    public static function create(
        array $data,
        string $iterator,
        bool $done,
    ): self {
        return new self(
            data: $data,
            iterator: $iterator,
            done: $done,
            setFields: ['data' => true, 'iterator' => true, 'done' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'iterator' => $this->iterator,
            'done' => $this->done];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'EventStreamOut', [EventOut::class, 'fromMixed']),
            iterator: \Svix\Utils::deserializeString($data, 'iterator', true, 'EventStreamOut'),
            done: \Svix\Utils::deserializeBool($data, 'done', true, 'EventStreamOut')
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
