<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $name         The event type's name
     * @param array|null        $schemas      the schema for the event type for a specific version as a JSON schema
     * @param string|null       $groupName    The event type group's name
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly string $name,
        public readonly string $description,
        public readonly bool $deprecated,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?bool $archived = null,
        public readonly ?array $schemas = null,
        public readonly ?string $groupName = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $featureFlag = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeOut with required fields.
     */
    public static function create(
        string $name,
        string $description,
        bool $deprecated,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            name: $name,
            description: $description,
            archived: null,
            deprecated: $deprecated,
            schemas: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            groupName: null,
            featureFlags: null,
            featureFlag: null,
            setFields: ['name' => true, 'description' => true, 'deprecated' => true, 'createdAt' => true, 'updatedAt' => true]
        );
    }

    public function withArchived(?bool $archived): self
    {
        $setFields = $this->setFields;
        $setFields['archived'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            archived: $archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
            featureFlag: $this->featureFlag,
            setFields: $setFields
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $schemas,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
            featureFlag: $this->featureFlag,
            setFields: $setFields
        );
    }

    public function withGroupName(?string $groupName): self
    {
        $setFields = $this->setFields;
        $setFields['groupName'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            groupName: $groupName,
            featureFlags: $this->featureFlags,
            featureFlag: $this->featureFlag,
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
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            groupName: $this->groupName,
            featureFlags: $featureFlags,
            featureFlag: $this->featureFlag,
            setFields: $setFields
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
            featureFlag: $featureFlag,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'description' => $this->description,
            'deprecated' => $this->deprecated,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c')];

        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (isset($this->setFields['schemas'])) {
            $data['schemas'] = $this->schemas;
        }
        if (isset($this->setFields['groupName'])) {
            $data['groupName'] = $this->groupName;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['featureFlag'])) {
            $data['featureFlag'] = $this->featureFlag;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'EventTypeOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EventTypeOut'),
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'EventTypeOut'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'EventTypeOut'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypeOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'EventTypeOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'EventTypeOut'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypeOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypeOut'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'EventTypeOut')
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
