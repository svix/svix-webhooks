<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id       the stream's ID
     * @param string|null           $uid      the stream's UID
     * @param string|null           $name     the stream's name
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly string $id,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly array $metadata,
        public readonly ?string $uid = null,
        public readonly ?string $name = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamOut with required fields.
     */
    public static function create(
        string $id,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        array $metadata,
    ): self {
        return new self(
            id: $id,
            uid: null,
            name: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            metadata: $metadata,
            setFields: ['id' => true, 'createdAt' => true, 'updatedAt' => true, 'metadata' => true]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            id: $this->id,
            uid: $uid,
            name: $this->name,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            id: $this->id,
            uid: $this->uid,
            name: $name,
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
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'metadata' => $this->metadata];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['name'])) {
            $data['name'] = $this->name;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'StreamOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamOut'),
            name: \Svix\Utils::deserializeString($data, 'name', false, 'StreamOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'StreamOut')
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
