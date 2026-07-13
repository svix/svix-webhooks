<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypePatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param string|null       $groupName    The event type group's name
     */
    private function __construct(
        public readonly ?string $description = null,
        public readonly ?bool $archived = null,
        public readonly ?bool $deprecated = null,
        public readonly ?array $schemas = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $groupName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypePatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            description: null,
            archived: null,
            deprecated: null,
            schemas: null,
            featureFlags: null,
            groupName: null,
            setFields: []
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            setFields: $setFields
        );
    }

    public function withArchived(?bool $archived): self
    {
        $setFields = $this->setFields;
        $setFields['archived'] = true;

        return new self(
            description: $this->description,
            archived: $archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            setFields: $setFields
        );
    }

    public function withDeprecated(?bool $deprecated): self
    {
        $setFields = $this->setFields;
        $setFields['deprecated'] = true;

        return new self(
            description: $this->description,
            archived: $this->archived,
            deprecated: $deprecated,
            schemas: $this->schemas,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            setFields: $setFields
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $schemas,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            featureFlags: $featureFlags,
            groupName: $this->groupName,
            setFields: $setFields
        );
    }

    public function withGroupName(?string $groupName): self
    {
        $setFields = $this->setFields;
        $setFields['groupName'] = true;

        return new self(
            description: $this->description,
            archived: $this->archived,
            deprecated: $this->deprecated,
            schemas: $this->schemas,
            featureFlags: $this->featureFlags,
            groupName: $groupName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (null !== $this->deprecated) {
            $data['deprecated'] = $this->deprecated;
        }
        if (isset($this->setFields['schemas'])) {
            $data['schemas'] = $this->schemas;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['groupName'])) {
            $data['groupName'] = $this->groupName;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EventTypePatch'),
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'EventTypePatch'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', false, 'EventTypePatch'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypePatch'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypePatch'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypePatch')
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
