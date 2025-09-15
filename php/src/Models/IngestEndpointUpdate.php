<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestEndpointUpdate implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $metadata
     * @param string|null                $uid      optional unique identifier for the endpoint
     */
    private function __construct(
        public readonly string $url,
        public readonly ?string $description = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $metadata = null,
        public readonly ?int $rateLimit = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointUpdate with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            description: null,
            disabled: null,
            metadata: null,
            rateLimit: null,
            uid: null,
            url: $url,
            setFields: ['url' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            disabled: $this->disabled,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
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
            description: $this->description,
            disabled: $disabled,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
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
            description: $this->description,
            disabled: $this->disabled,
            metadata: $metadata,
            rateLimit: $this->rateLimit,
            uid: $this->uid,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            description: $this->description,
            disabled: $this->disabled,
            metadata: $this->metadata,
            rateLimit: $rateLimit,
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
            description: $this->description,
            disabled: $this->disabled,
            metadata: $this->metadata,
            rateLimit: $this->rateLimit,
            uid: $uid,
            url: $this->url,
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
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
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
            description: \Svix\Utils::deserializeString($data, 'description', false, 'IngestEndpointUpdate'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'IngestEndpointUpdate'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'IngestEndpointUpdate'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'IngestEndpointUpdate'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'IngestEndpointUpdate'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'IngestEndpointUpdate')
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
