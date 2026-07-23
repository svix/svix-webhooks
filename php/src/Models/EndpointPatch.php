<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null                $uid        the Endpoint's UID
     * @param list<string>|null          $eventTypes
     * @param list<string>|null          $channels
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly ?string $description = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?string $url = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $channels = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            description: null,
            throttleRate: null,
            uid: null,
            url: null,
            disabled: null,
            eventTypes: null,
            channels: null,
            metadata: null,
            setFields: []
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $eventTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->url) {
            $data['url'] = $this->url;
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
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EndpointPatch'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'EndpointPatch'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'EndpointPatch'),
            url: \Svix\Utils::getValFromJson($data, 'url', false, 'EndpointPatch'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'EndpointPatch'),
            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'EndpointPatch'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointPatch'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'EndpointPatch')
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
