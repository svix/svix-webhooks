<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamEventTypeOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $name         The event type's name
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly string $name,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly bool $deprecated,
        public readonly bool $archived,
        public readonly ?string $description = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamEventTypeOut with required fields.
     */
    public static function create(
        string $name,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        bool $deprecated,
        bool $archived,
    ): self {
        return new self(
            name: $name,
            description: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            deprecated: $deprecated,
            archived: $archived,
            featureFlags: null,
            setFields: ['name' => true, 'createdAt' => true, 'updatedAt' => true, 'deprecated' => true, 'archived' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            name: $this->name,
            description: $description,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            deprecated: $this->deprecated,
            archived: $this->archived,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            deprecated: $this->deprecated,
            archived: $this->archived,
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'deprecated' => $this->deprecated,
            'archived' => $this->archived];

        if (isset($this->setFields['description'])) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'StreamEventTypeOut'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'StreamEventTypeOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamEventTypeOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamEventTypeOut'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'StreamEventTypeOut'),
            archived: \Svix\Utils::deserializeBool($data, 'archived', true, 'StreamEventTypeOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'StreamEventTypeOut')
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
