<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointUpsertIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null                $uid         optional unique identifier for the endpoint
     * @param list<string>|null          $filterTypes
     * @param list<string>|null          $channels    list of message channels this endpoint listens to (omit for all)
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly string $url,
        public readonly ?string $description = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?array $channels = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointUpsertIn with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            description: null,
            throttleRate: null,
            uid: null,
            url: $url,
            disabled: null,
            filterTypes: null,
            channels: null,
            metadata: null,
            setFields: ['url' => true]
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
            filterTypes: $this->filterTypes,
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
            filterTypes: $this->filterTypes,
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
            filterTypes: $this->filterTypes,
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
            filterTypes: $this->filterTypes,
            channels: $this->channels,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
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
            filterTypes: $this->filterTypes,
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
            filterTypes: $this->filterTypes,
            channels: $this->channels,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
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
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EndpointUpsertIn'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'EndpointUpsertIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'EndpointUpsertIn'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'EndpointUpsertIn'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'EndpointUpsertIn'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'EndpointUpsertIn'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'EndpointUpsertIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'EndpointUpsertIn')
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
