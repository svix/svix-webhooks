<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id       the stream's ID
     * @param array<string, string> $metadata
     * @param string|null           $name     the stream's name
     * @param string|null           $uid      the stream's UID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $id,
        public readonly array $metadata,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?string $name = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $id,
        array $metadata,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            createdAt: $createdAt,
            id: $id,
            metadata: $metadata,
            name: null,
            uid: null,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'id' => true, 'metadata' => true, 'updatedAt' => true]
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            createdAt: $this->createdAt,
            id: $this->id,
            metadata: $this->metadata,
            name: $name,
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
            uid: $uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'id' => $this->id,
            'metadata' => $this->metadata,
            'updatedAt' => $this->updatedAt->format('c')];

        if (isset($this->setFields['name'])) {
            $data['name'] = $this->name;
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
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'StreamOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'StreamOut'),
            name: \Svix\Utils::deserializeString($data, 'name', false, 'StreamOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamOut')
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
