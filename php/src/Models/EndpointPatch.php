<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null          $channels
     * @param list<string>|null          $filterTypes
     * @param array<string, string>|null $metadata
     * @param int|null                   $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null $uid the Endpoint's UID
     */
    private function __construct(
        public readonly ?array $channels = null,
        public readonly ?string $description = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?array $metadata = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?string $url = null,
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
            channels: null,
            description: null,
            disabled: null,
            filterTypes: null,
            metadata: null,
            throttleRate: null,
            uid: null,
            url: null,
            setFields: []
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            channels: $channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            channels: $this->channels,
            description: $description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $throttleRate,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            channels: $this->channels,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            metadata: $this->metadata,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $url,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointPatch'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EndpointPatch'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'EndpointPatch'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'EndpointPatch'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'EndpointPatch'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'EndpointPatch'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'EndpointPatch'),
            url: \Svix\Utils::getValFromJson($data, 'url', false, 'EndpointPatch')
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
