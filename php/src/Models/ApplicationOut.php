<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id       the Application's ID
     * @param array<string, string> $metadata
     * @param string|null           $uid      the Application's UID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $id,
        public readonly array $metadata,
        public readonly string $name,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?int $rateLimit = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $id,
        array $metadata,
        string $name,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            createdAt: $createdAt,
            id: $id,
            metadata: $metadata,
            name: $name,
            rateLimit: null,
            uid: null,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'id' => true, 'metadata' => true, 'name' => true, 'updatedAt' => true]
        );
    }

    public function withRateLimit(?int $rateLimit): self
    {
        $setFields = $this->setFields;
        $setFields['rateLimit'] = true;

        return new self(
            createdAt: $this->createdAt,
            id: $this->id,
            metadata: $this->metadata,
            name: $this->name,
            rateLimit: $rateLimit,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            createdAt: $this->createdAt,
            id: $this->id,
            metadata: $this->metadata,
            name: $this->name,
            rateLimit: $this->rateLimit,
            uid: $uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['createdAt' => $this->createdAt->format('c'),
            'id' => $this->id,
            'metadata' => $this->metadata,
            'name' => $this->name,
            'updatedAt' => $this->updatedAt->format('c')];

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
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ApplicationOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ApplicationOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'ApplicationOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ApplicationOut'),
            rateLimit: \Svix\Utils::deserializeInt($data, 'rateLimit', false, 'ApplicationOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ApplicationOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'ApplicationOut')
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
