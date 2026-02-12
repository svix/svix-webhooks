<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $channels     list of message channels this endpoint listens to (omit for all)
     * @param string            $description  an example endpoint name
     * @param list<string>|null $filterTypes
     * @param string            $id           the Endpoint's ID
     * @param int|null          $rateLimit    deprecated, use `throttleRate` instead
     * @param int|null          $throttleRate Maximum messages per second to send to this endpoint. Outgoing messages will be throttled to this rate.
     * @param string|null       $uid          optional unique identifier for the endpoint
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $description,
        public readonly string $id,
        public readonly MessageStatus $status,
        public readonly MessageStatusText $statusText,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly string $url,
        public readonly int $version,
        public readonly ?array $channels = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?\DateTimeImmutable $nextAttempt = null,
        public readonly ?int $rateLimit = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageEndpointOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $description,
        string $id,
        MessageStatus $status,
        MessageStatusText $statusText,
        \DateTimeImmutable $updatedAt,
        string $url,
        int $version,
    ): self {
        return new self(
            channels: null,
            createdAt: $createdAt,
            description: $description,
            disabled: null,
            filterTypes: null,
            id: $id,
            nextAttempt: null,
            rateLimit: null,
            status: $status,
            statusText: $statusText,
            throttleRate: null,
            uid: null,
            updatedAt: $updatedAt,
            url: $url,
            version: $version,
            setFields: ['createdAt' => true, 'description' => true, 'id' => true, 'status' => true, 'statusText' => true, 'updatedAt' => true, 'url' => true, 'version' => true]
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            channels: $channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withNextAttempt(?\DateTimeImmutable $nextAttempt): self
    {
        $setFields = $this->setFields;
        $setFields['nextAttempt'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $throttleRate,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            channels: $this->channels,
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            nextAttempt: $this->nextAttempt,
            rateLimit: $this->rateLimit,
            status: $this->status,
            statusText: $this->statusText,
            throttleRate: $this->throttleRate,
            uid: $uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            version: $this->version,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'description' => $this->description,
            'id' => $this->id,
            'status' => $this->status,
            'statusText' => $this->statusText,
            'updatedAt' => $this->updatedAt->format('c'),
            'url' => $this->url,
            'version' => $this->version];

        if (isset($this->setFields['channels'])) {
            $data['channels'] = $this->channels;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (isset($this->setFields['nextAttempt'])) {
            $data['nextAttempt'] = $this->nextAttempt->format('c');
        }
        if (isset($this->setFields['rateLimit'])) {
            $data['rateLimit'] = $this->rateLimit;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'MessageEndpointOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'MessageEndpointOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'MessageEndpointOut'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'MessageEndpointOut'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'MessageEndpointOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'MessageEndpointOut'),
            nextAttempt: \Svix\Utils::deserializeDt($data, 'nextAttempt', false, 'MessageEndpointOut'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'MessageEndpointOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'MessageEndpointOut', [MessageStatus::class, 'fromMixed']),
            statusText: \Svix\Utils::deserializeObject($data, 'statusText', true, 'MessageEndpointOut', [MessageStatusText::class, 'fromMixed']),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'MessageEndpointOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'MessageEndpointOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'MessageEndpointOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'MessageEndpointOut'),
            version: \Svix\Utils::deserializeInt($data, 'version', true, 'MessageEndpointOut')
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
