<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessagePrecheckIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $channels
     * @param string            $eventType The event type's name
     */
    private function __construct(
        public readonly string $eventType,
        public readonly ?array $channels = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessagePrecheckIn with required fields.
     */
    public static function create(
        string $eventType,
    ): self {
        return new self(
            channels: null,
            eventType: $eventType,
            setFields: ['eventType' => true]
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            channels: $channels,
            eventType: $this->eventType,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventType' => $this->eventType];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'MessagePrecheckIn'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'MessagePrecheckIn')
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
