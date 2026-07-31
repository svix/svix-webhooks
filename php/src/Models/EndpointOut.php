<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id           the Endpoint's ID
     * @param array<string, string> $metadata
     * @param int|null              $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null       $uid        optional unique identifier for the endpoint
     * @param list<string>|null $eventTypes
     * @param list<string>|null $channels   list of message channels this endpoint listens to (omit for all)
     */
    private function __construct(
        public readonly string $id,
        public readonly array $metadata,
        public readonly string $url,
        public readonly string $description,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $channels = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointOut with required fields.
     */
    public static function create(
        string $id,
        array $metadata,
        string $url,
        string $description,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            id: $id,
            metadata: $metadata,
            url: $url,
            description: $description,
            throttleRate: null,
            uid: null,
            disabled: null,
            eventTypes: null,
            channels: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            setFields: ['id' => true, 'metadata' => true, 'url' => true, 'description' => true, 'createdAt' => true, 'updatedAt' => true]
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            id: $this->id,
            metadata: $this->metadata,
            url: $this->url,
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            id: $this->id,
            metadata: $this->metadata,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            id: $this->id,
            metadata: $this->metadata,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            id: $this->id,
            metadata: $this->metadata,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            eventTypes: $eventTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            id: $this->id,
            metadata: $this->metadata,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'metadata' => $this->metadata,
            'url' => $this->url,
            'description' => $this->description,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c')];

        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['eventTypes'])) {
            $data['eventTypes'] = $this->eventTypes;
        }
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
            id: \Svix\Utils::deserializeString($data, 'id', true, 'EndpointOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'EndpointOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'EndpointOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EndpointOut'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'EndpointOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'EndpointOut'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'EndpointOut'),
            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'EndpointOut'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'EndpointOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'EndpointOut')
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
