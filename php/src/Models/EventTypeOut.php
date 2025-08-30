<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param string|null       $groupName    The event type group's name
     * @param string            $name         The event type's name
     * @param array|null        $schemas      the schema for the event type for a specific version as a JSON schema
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly bool $deprecated,
        public readonly string $description,
        public readonly string $name,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?bool $archived = null,
        public readonly ?string $featureFlag = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $groupName = null,
        public readonly ?array $schemas = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        bool $deprecated,
        string $description,
        string $name,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            archived: null,
            createdAt: $createdAt,
            deprecated: $deprecated,
            description: $description,
            featureFlag: null,
            featureFlags: null,
            groupName: null,
            name: $name,
            schemas: null,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'deprecated' => true, 'description' => true, 'name' => true, 'updatedAt' => true]
        );
    }

    public function withArchived(?bool $archived): self
    {
        $setFields = $this->setFields;
        $setFields['archived'] = true;

        return new self(
            archived: $archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $this->schemas,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            archived: $this->archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $this->schemas,
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
            featureFlag: $this->featureFlag,
            featureFlags: $featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $this->schemas,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withGroupName(?string $groupName): self
    {
        $setFields = $this->setFields;
        $setFields['groupName'] = true;

        return new self(
            archived: $this->archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $groupName,
            name: $this->name,
            schemas: $this->schemas,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            archived: $this->archived,
            createdAt: $this->createdAt,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $schemas,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['createdAt' => $this->createdAt->format('c'),
            'deprecated' => $this->deprecated,
            'description' => $this->description,
            'name' => $this->name,
            'updatedAt' => $this->updatedAt->format('c')];

        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (isset($this->setFields['featureFlag'])) {
            $data['featureFlag'] = $this->featureFlag;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['groupName'])) {
            $data['groupName'] = $this->groupName;
        }
        if (isset($this->setFields['schemas'])) {
            $data['schemas'] = $this->schemas;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'EventTypeOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'EventTypeOut'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'EventTypeOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EventTypeOut'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'EventTypeOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypeOut'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypeOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'EventTypeOut'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypeOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'EventTypeOut')
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
