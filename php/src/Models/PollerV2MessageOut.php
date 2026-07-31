<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** The MessageOut equivalent of polling endpoint */
class PollerV2MessageOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $headers
     * @param string|null                $eventId   Optional unique identifier for the message
     * @param string                     $eventType The event type's name
     * @param list<string>|null          $channels  List of free-form identifiers that endpoints can filter by
     * @param string                     $id        the Message's ID
     * @param list<string>|null          $tags
     */
    private function __construct(
        public readonly int $offset,
        public readonly string $eventType,
        public readonly array $payload,
        public readonly string $id,
        public readonly \DateTimeImmutable $timestamp,
        public readonly ?array $headers = null,
        public readonly ?string $eventId = null,
        public readonly ?array $channels = null,
        public readonly ?array $tags = null,
        public readonly ?\DateTimeImmutable $deliverAt = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollerV2MessageOut with required fields.
     */
    public static function create(
        int $offset,
        string $eventType,
        array $payload,
        string $id,
        \DateTimeImmutable $timestamp,
    ): self {
        return new self(
            offset: $offset,
            headers: null,
            eventId: null,
            eventType: $eventType,
            payload: $payload,
            channels: null,
            id: $id,
            timestamp: $timestamp,
            tags: null,
            deliverAt: null,
            setFields: ['offset' => true, 'eventType' => true, 'payload' => true, 'id' => true, 'timestamp' => true]
        );
    }

    public function withHeaders(?array $headers): self
    {
        $setFields = $this->setFields;
        $setFields['headers'] = true;

        return new self(
            offset: $this->offset,
            headers: $headers,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            id: $this->id,
            timestamp: $this->timestamp,
            tags: $this->tags,
            deliverAt: $this->deliverAt,
            setFields: $setFields
        );
    }

    public function withEventId(?string $eventId): self
    {
        $setFields = $this->setFields;
        $setFields['eventId'] = true;

        return new self(
            offset: $this->offset,
            headers: $this->headers,
            eventId: $eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            id: $this->id,
            timestamp: $this->timestamp,
            tags: $this->tags,
            deliverAt: $this->deliverAt,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            offset: $this->offset,
            headers: $this->headers,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $channels,
            id: $this->id,
            timestamp: $this->timestamp,
            tags: $this->tags,
            deliverAt: $this->deliverAt,
            setFields: $setFields
        );
    }

    public function withTags(?array $tags): self
    {
        $setFields = $this->setFields;
        $setFields['tags'] = true;

        return new self(
            offset: $this->offset,
            headers: $this->headers,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            id: $this->id,
            timestamp: $this->timestamp,
            tags: $tags,
            deliverAt: $this->deliverAt,
            setFields: $setFields
        );
    }

    public function withDeliverAt(?\DateTimeImmutable $deliverAt): self
    {
        $setFields = $this->setFields;
        $setFields['deliverAt'] = true;

        return new self(
            offset: $this->offset,
            headers: $this->headers,
            eventId: $this->eventId,
            eventType: $this->eventType,
            payload: $this->payload,
            channels: $this->channels,
            id: $this->id,
            timestamp: $this->timestamp,
            tags: $this->tags,
            deliverAt: $deliverAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'offset' => $this->offset,
            'eventType' => $this->eventType,
            'payload' => $this->payload,
            'id' => $this->id,
            'timestamp' => $this->timestamp->format('c')];

        if (isset($this->setFields['headers'])) {
            $data['headers'] = $this->headers;
        }
        if (isset($this->setFields['eventId'])) {
            $data['eventId'] = $this->eventId;
        }
        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['tags'])) {
            $data['tags'] = $this->tags;
        }
        if (isset($this->setFields['deliverAt'])) {
            $data['deliverAt'] = $this->deliverAt->format('c');
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            offset: \Svix\Utils::deserializeInt($data, 'offset', true, 'PollerV2MessageOut'),
            headers: \Svix\Utils::getValFromJson($data, 'headers', false, 'PollerV2MessageOut'),
            eventId: \Svix\Utils::deserializeString($data, 'eventId', false, 'PollerV2MessageOut'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'PollerV2MessageOut'),
            payload: \Svix\Utils::getValFromJson($data, 'payload', true, 'PollerV2MessageOut'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'PollerV2MessageOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'PollerV2MessageOut'),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'PollerV2MessageOut'),
            tags: \Svix\Utils::getValFromJson($data, 'tags', false, 'PollerV2MessageOut'),
            deliverAt: \Svix\Utils::deserializeDt($data, 'deliverAt', false, 'PollerV2MessageOut')
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
