<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamEventTypeOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param string            $name         The event type's name
     */
    private function __construct(
        public readonly bool $archived,
        public readonly \DateTimeImmutable $createdAt,
        public readonly bool $deprecated,
        public readonly string $name,
        public readonly \DateTimeImmutable $updatedAt,
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
        bool $archived,
        \DateTimeImmutable $createdAt,
        bool $deprecated,
        string $name,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            archived: $archived,
            createdAt: $createdAt,
            deprecated: $deprecated,
            description: null,
            featureFlags: null,
            name: $name,
            updatedAt: $updatedAt,
            setFields: ['archived' => true, 'createdAt' => true, 'deprecated' => true, 'name' => true, 'updatedAt' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            archived: $this->archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $description,
            featureFlags: $this->featureFlags,
            name: $this->name,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            archived: $this->archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlags: $featureFlags,
            name: $this->name,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'archived' => $this->archived,
            'createdAt' => $this->createdAt->format('c'),
            'deprecated' => $this->deprecated,
            'name' => $this->name,
            'updatedAt' => $this->updatedAt->format('c')];

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
            archived: \Svix\Utils::deserializeBool($data, 'archived', true, 'StreamEventTypeOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamEventTypeOut'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'StreamEventTypeOut'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'StreamEventTypeOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'StreamEventTypeOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'StreamEventTypeOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamEventTypeOut')
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
