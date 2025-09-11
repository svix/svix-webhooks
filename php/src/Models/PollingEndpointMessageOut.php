<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** The MessageOut equivalent of polling endpoint */
class PollingEndpointMessageOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null          $channels  List of free-form identifiers that endpoints can filter by
     * @param string|null                $eventId   Optional unique identifier for the message
     * @param string                     $eventType The event type's name
     * @param array<string, string>|null $headers
     * @param string                     $id        the Message's ID
     * @param list<string>|null          $tags
     */
    private function __construct(
        public readonly string $eventType,
        public readonly string $id,
        public readonly array $payload,
        public readonly \DateTimeImmutable $timestamp,
        public readonly ?array $channels = null,
        public readonly ?string $eventId = null,
        public readonly ?array $headers = null,
        public readonly ?array $tags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollingEndpointMessageOut with required fields.
     */
    public static function create(
        string $eventType,
        string $id,
        array $payload,
        \DateTimeImmutable $timestamp,
    ): self {
        return new self(
            channels: null,
            eventId: null,
            eventType: $eventType,
            headers: null,
            id: $id,
            payload: $payload,
            tags: null,
            timestamp: $timestamp,
            setFields: ['eventType' => true, 'id' => true, 'payload' => true, 'timestamp' => true]
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            channels: $channels,
            eventId: $this->eventId,
            eventType: $this->eventType,
            headers: $this->headers,
            id: $this->id,
            payload: $this->payload,
            tags: $this->tags,
            timestamp: $this->timestamp,
            setFields: $setFields
        );
    }

    public function withEventId(?string $eventId): self
    {
        $setFields = $this->setFields;
        $setFields['eventId'] = true;

        return new self(
            channels: $this->channels,
            eventId: $eventId,
            eventType: $this->eventType,
            headers: $this->headers,
            id: $this->id,
            payload: $this->payload,
            tags: $this->tags,
            timestamp: $this->timestamp,
            setFields: $setFields
        );
    }

    public function withHeaders(?array $headers): self
    {
        $setFields = $this->setFields;
        $setFields['headers'] = true;

        return new self(
            channels: $this->channels,
            eventId: $this->eventId,
            eventType: $this->eventType,
            headers: $headers,
            id: $this->id,
            payload: $this->payload,
            tags: $this->tags,
            timestamp: $this->timestamp,
            setFields: $setFields
        );
    }

    public function withTags(?array $tags): self
    {
        $setFields = $this->setFields;
        $setFields['tags'] = true;

        return new self(
            channels: $this->channels,
            eventId: $this->eventId,
            eventType: $this->eventType,
            headers: $this->headers,
            id: $this->id,
            payload: $this->payload,
            tags: $tags,
            timestamp: $this->timestamp,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventType' => $this->eventType,
            'id' => $this->id,
            'payload' => $this->payload,
            'timestamp' => $this->timestamp->format('c')];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['eventId'])) {
            $data['eventId'] = $this->eventId;
        }
        if (isset($this->setFields['headers'])) {
            $data['headers'] = $this->headers;
        }
        if (isset($this->setFields['tags'])) {
            $data['tags'] = $this->tags;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'PollingEndpointMessageOut'),
            eventId: \Svix\Utils::deserializeString($data, 'eventId', false, 'PollingEndpointMessageOut'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'PollingEndpointMessageOut'),
            headers: \Svix\Utils::getValFromJson($data, 'headers', false, 'PollingEndpointMessageOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'PollingEndpointMessageOut'),
            payload: \Svix\Utils::getValFromJson($data, 'payload', true, 'PollingEndpointMessageOut'),
            tags: \Svix\Utils::getValFromJson($data, 'tags', false, 'PollingEndpointMessageOut'),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'PollingEndpointMessageOut')
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
