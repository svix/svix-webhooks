<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** A model containing information on a given message plus additional fields on the last attempt for that message. */
class EndpointMessageOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $channels  List of free-form identifiers that endpoints can filter by
     * @param string|null       $eventId   Optional unique identifier for the message
     * @param string            $eventType The event type's name
     * @param string            $id        the Message's ID
     * @param list<string>|null $tags
     */
    private function __construct(
        public readonly string $eventType,
        public readonly string $id,
        public readonly array $payload,
        public readonly MessageStatus $status,
        public readonly MessageStatusText $statusText,
        public readonly \DateTimeImmutable $timestamp,
        public readonly ?array $channels = null,
        public readonly ?string $eventId = null,
        public readonly ?\DateTimeImmutable $nextAttempt = null,
        public readonly ?array $tags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointMessageOut with required fields.
     */
    public static function create(
        string $eventType,
        string $id,
        array $payload,
        MessageStatus $status,
        MessageStatusText $statusText,
        \DateTimeImmutable $timestamp,
    ): self {
        return new self(
            channels: null,
            eventId: null,
            eventType: $eventType,
            id: $id,
            nextAttempt: null,
            payload: $payload,
            status: $status,
            statusText: $statusText,
            tags: null,
            timestamp: $timestamp,
            setFields: ['eventType' => true, 'id' => true, 'payload' => true, 'status' => true, 'statusText' => true, 'timestamp' => true]
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
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            payload: $this->payload,
            status: $this->status,
            statusText: $this->statusText,
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
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            payload: $this->payload,
            status: $this->status,
            statusText: $this->statusText,
            tags: $this->tags,
            timestamp: $this->timestamp,
            setFields: $setFields
        );
    }

    public function withNextAttempt(?\DateTimeImmutable $nextAttempt): self
    {
        $setFields = $this->setFields;
        $setFields['nextAttempt'] = true;

        return new self(
            channels: $this->channels,
            eventId: $this->eventId,
            eventType: $this->eventType,
            id: $this->id,
            nextAttempt: $nextAttempt,
            payload: $this->payload,
            status: $this->status,
            statusText: $this->statusText,
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
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            payload: $this->payload,
            status: $this->status,
            statusText: $this->statusText,
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
            'status' => $this->status,
            'statusText' => $this->statusText,
            'timestamp' => $this->timestamp->format('c')];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['eventId'])) {
            $data['eventId'] = $this->eventId;
        }
        if (isset($this->setFields['nextAttempt'])) {
            $data['nextAttempt'] = $this->nextAttempt->format('c');
        }
        if (isset($this->setFields['tags'])) {
            $data['tags'] = $this->tags;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointMessageOut'),
            eventId: \Svix\Utils::deserializeString($data, 'eventId', false, 'EndpointMessageOut'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', true, 'EndpointMessageOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'EndpointMessageOut'),
            nextAttempt: \Svix\Utils::deserializeDt($data, 'nextAttempt', false, 'EndpointMessageOut'),
            payload: \Svix\Utils::getValFromJson($data, 'payload', true, 'EndpointMessageOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'EndpointMessageOut', [MessageStatus::class, 'fromMixed']),
            statusText: \Svix\Utils::deserializeObject($data, 'statusText', true, 'EndpointMessageOut', [MessageStatusText::class, 'fromMixed']),
            tags: \Svix\Utils::getValFromJson($data, 'tags', false, 'EndpointMessageOut'),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'EndpointMessageOut')
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
