<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $uid          optional unique identifier for the application
     * @param string      $name         application name for human consumption
     * @param int|null    $throttleRate Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string                $id       the Application's ID
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly string $name,
        public readonly string $id,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly array $metadata,
        public readonly ?string $uid = null,
        public readonly ?int $throttleRate = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationOut with required fields.
     */
    public static function create(
        string $name,
        string $id,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        array $metadata,
    ): self {
        return new self(
            uid: null,
            name: $name,
            throttleRate: null,
            id: $id,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            metadata: $metadata,
            setFields: ['name' => true, 'id' => true, 'createdAt' => true, 'updatedAt' => true, 'metadata' => true]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            uid: $uid,
            name: $this->name,
            throttleRate: $this->throttleRate,
            id: $this->id,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            uid: $this->uid,
            name: $this->name,
            throttleRate: $throttleRate,
            id: $this->id,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'id' => $this->id,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'metadata' => $this->metadata];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ApplicationOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ApplicationOut'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'ApplicationOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ApplicationOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ApplicationOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'ApplicationOut'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'ApplicationOut')
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
