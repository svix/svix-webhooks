<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $metadata
     * @param string|null                $uid      optional unique identifier for the application
     */
    private function __construct(
        public readonly string $name,
        public readonly ?array $metadata = null,
        public readonly ?int $rateLimit = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationIn with required fields.
     */
    public static function create(
        string $name,
    ): self {
        return new self(
            metadata: null,
            name: $name,
            rateLimit: null,
            uid: null,
            setFields: ['name' => true]
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            metadata: $metadata,
            name: $this->name,
            rateLimit: $this->rateLimit,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            metadata: $this->metadata,
            name: $this->name,
            rateLimit: $rateLimit,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            metadata: $this->metadata,
            name: $this->name,
            rateLimit: $this->rateLimit,
            uid: $uid,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['name' => $this->name];

        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }
        if (isset($this->setFields['rateLimit'])) {
            $data['rateLimit'] = $this->rateLimit;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'ApplicationIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ApplicationIn'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'ApplicationIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ApplicationIn')
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
