<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventExampleIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string   $eventType    The event type's name
     * @param int|null $exampleIndex If the event type schema contains an array of examples, chooses which one to send.
     *
     * Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
     */
    private function __construct(
        public readonly string $eventType,
        public readonly ?int $exampleIndex = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventExampleIn with required fields.
     */
    public static function create(
        string $eventType,
    ): self {
        return new self(
            eventType: $eventType,
            exampleIndex: null,
            setFields: ['eventType' => true]
        );
    }

    public function withExampleIndex(?int $exampleIndex): self
    {
        $setFields = $this->setFields;
        $setFields['exampleIndex'] = true;

        return new self(
            eventType: $this->eventType,
            exampleIndex: $exampleIndex,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['eventType' => $this->eventType];

        if (null !== $this->exampleIndex) {
            $data['exampleIndex'] = $this->exampleIndex;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'EventExampleIn'),
            exampleIndex: \Svix\Utils::deserializeInt($data, 'exampleIndex', false, 'EventExampleIn')
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
