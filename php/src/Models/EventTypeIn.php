<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeIn implements \JsonSerializable
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
        public readonly ?bool $archived = null,
        public readonly ?bool $deprecated = null,
        public readonly ?array $schemas = null,
        public readonly ?string $groupName = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeIn with required fields.
     */
    public static function create(
        string $name,
        string $description,
    ): self {
        return new self(
            name: $name,
            description: $description,
            archived: null,
            deprecated: null,
            schemas: null,
            groupName: null,
            featureFlags: null,
            setFields: ['name' => true, 'description' => true]
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
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withDeprecated(?bool $deprecated): self
    {
        $setFields = $this->setFields;
        $setFields['deprecated'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            archived: $this->archived,
            deprecated: $deprecated,
            schemas: $this->schemas,
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
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
            groupName: $this->groupName,
            featureFlags: $this->featureFlags,
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
            groupName: $groupName,
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
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            groupName: $this->groupName,
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'description' => $this->description];

        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (null !== $this->deprecated) {
            $data['deprecated'] = $this->deprecated;
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'EventTypeIn'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EventTypeIn'),
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'EventTypeIn'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', false, 'EventTypeIn'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypeIn'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypeIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypeIn')
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
