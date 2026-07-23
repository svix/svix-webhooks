<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string   $id           the Endpoint's ID
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null           $uid      optional unique identifier for the endpoint
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly string $id,
        public readonly string $url,
        public readonly string $description,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly array $metadata,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointOut with required fields.
     */
    public static function create(
        string $id,
        string $url,
        string $description,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        array $metadata,
    ): self {
        return new self(
            id: $id,
            url: $url,
            description: $description,
            throttleRate: null,
            uid: null,
            disabled: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            metadata: $metadata,
            setFields: ['id' => true, 'url' => true, 'description' => true, 'createdAt' => true, 'updatedAt' => true, 'metadata' => true]
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            id: $this->id,
            url: $this->url,
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            disabled: $this->disabled,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            id: $this->id,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            disabled: $this->disabled,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            id: $this->id,
            url: $this->url,
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            disabled: $disabled,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'url' => $this->url,
            'description' => $this->description,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'metadata' => $this->metadata];

        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'IngestEndpointOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'IngestEndpointOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'IngestEndpointOut'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'IngestEndpointOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'IngestEndpointOut'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'IngestEndpointOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'IngestEndpointOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'IngestEndpointOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'IngestEndpointOut')
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
