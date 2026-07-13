<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string   $id           the Endpoint's ID
     * @param string   $description  an example endpoint name
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null       $uid         optional unique identifier for the endpoint
     * @param list<string>|null $filterTypes
     * @param list<string>|null $channels    list of message channels this endpoint listens to (omit for all)
     */
    private function __construct(
        public readonly string $id,
        public readonly MessageStatus $status,
        public readonly MessageStatusText $statusText,
        public readonly string $description,
        public readonly string $url,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?\DateTimeImmutable $nextAttempt = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?array $channels = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageEndpointOut with required fields.
     */
    public static function create(
        string $id,
        MessageStatus $status,
        MessageStatusText $statusText,
        string $description,
        string $url,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            id: $id,
            status: $status,
            statusText: $statusText,
            nextAttempt: null,
            description: $description,
            throttleRate: null,
            uid: null,
            url: $url,
            disabled: null,
            filterTypes: null,
            channels: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            setFields: ['id' => true, 'status' => true, 'statusText' => true, 'description' => true, 'url' => true, 'createdAt' => true, 'updatedAt' => true]
        );
    }

    public function withNextAttempt(?\DateTimeImmutable $nextAttempt): self
    {
        $setFields = $this->setFields;
        $setFields['nextAttempt'] = true;

        return new self(
            id: $this->id,
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $nextAttempt,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            id: $this->id,
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $this->nextAttempt,
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
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
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $this->nextAttempt,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
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
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $this->nextAttempt,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $disabled,
            filterTypes: $this->filterTypes,
            channels: $this->channels,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            id: $this->id,
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $this->nextAttempt,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
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
            status: $this->status,
            statusText: $this->statusText,
            nextAttempt: $this->nextAttempt,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
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
            'status' => $this->status,
            'statusText' => $this->statusText,
            'description' => $this->description,
            'url' => $this->url,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c')];

        if (isset($this->setFields['nextAttempt'])) {
            $data['nextAttempt'] = $this->nextAttempt->format('c');
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'MessageEndpointOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'MessageEndpointOut', [MessageStatus::class, 'fromMixed']),
            statusText: \Svix\Utils::deserializeObject($data, 'statusText', true, 'MessageEndpointOut', [MessageStatusText::class, 'fromMixed']),
            nextAttempt: \Svix\Utils::deserializeDt($data, 'nextAttempt', false, 'MessageEndpointOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'MessageEndpointOut'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'MessageEndpointOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'MessageEndpointOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'MessageEndpointOut'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'MessageEndpointOut'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'MessageEndpointOut'),
            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'MessageEndpointOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'MessageEndpointOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'MessageEndpointOut')
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
