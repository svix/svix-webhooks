<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class OperationalWebhookEndpointOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $description an example endpoint name
     * @param list<string>|null     $filterTypes
     * @param string                $id          the Endpoint's ID
     * @param array<string, string> $metadata
     * @param string|null           $uid         optional unique identifier for the endpoint
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $description,
        public readonly string $id,
        public readonly array $metadata,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly string $url,
        public readonly ?bool $disabled = null,
        public readonly ?array $filterTypes = null,
        public readonly ?int $rateLimit = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of OperationalWebhookEndpointOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $description,
        string $id,
        array $metadata,
        \DateTimeImmutable $updatedAt,
        string $url,
    ): self {
        return new self(
            createdAt: $createdAt,
            description: $description,
            disabled: null,
            filterTypes: null,
            id: $id,
            metadata: $metadata,
            rateLimit: null,
            uid: null,
            updatedAt: $updatedAt,
            url: $url,
            setFields: ['createdAt' => true, 'description' => true, 'id' => true, 'metadata' => true, 'updatedAt' => true, 'url' => true]
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $filterTypes,
            id: $this->id,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            metadata: $this->metadata,
            rateLimit: $rateLimit,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            disabled: $this->disabled,
            filterTypes: $this->filterTypes,
            id: $this->id,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            uid: $uid,
            updatedAt: $this->updatedAt,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'description' => $this->description,
            'id' => $this->id,
            'metadata' => $this->metadata,
            'updatedAt' => $this->updatedAt->format('c'),
            'url' => $this->url];

        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (isset($this->setFields['rateLimit'])) {
            $data['rateLimit'] = $this->rateLimit;
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
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'OperationalWebhookEndpointOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'OperationalWebhookEndpointOut'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'OperationalWebhookEndpointOut'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'OperationalWebhookEndpointOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'OperationalWebhookEndpointOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'OperationalWebhookEndpointOut'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'OperationalWebhookEndpointOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'OperationalWebhookEndpointOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'OperationalWebhookEndpointOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'OperationalWebhookEndpointOut')
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
