<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypePatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null       $featureFlag  deprecated, use `featureFlags` instead
     * @param list<string>|null $featureFlags
     * @param string|null       $groupName    The event type group's name
     */
    private function __construct(
        public readonly ?bool $archived = null,
        public readonly ?bool $deprecated = null,
        public readonly ?string $description = null,
        public readonly ?string $featureFlag = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $groupName = null,
        public readonly ?array $schemas = null,
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
            archived: null,
            deprecated: null,
            description: null,
            featureFlag: null,
            featureFlags: null,
            groupName: null,
            schemas: null,
            setFields: []
        );
    }

    public function withArchived(?bool $archived): self
    {
        $setFields = $this->setFields;
        $setFields['archived'] = true;

        return new self(
            archived: $archived,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withDeprecated(?bool $deprecated): self
    {
        $setFields = $this->setFields;
        $setFields['deprecated'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $this->deprecated,
            description: $description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $featureFlags,
            groupName: $this->groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withGroupName(?string $groupName): self
    {
        $setFields = $this->setFields;
        $setFields['groupName'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $groupName,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            archived: $this->archived,
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            schemas: $schemas,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (null !== $this->deprecated) {
            $data['deprecated'] = $this->deprecated;
        }
        if (null !== $this->description) {
            $data['description'] = $this->description;
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'EventTypePatch'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', false, 'EventTypePatch'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'EventTypePatch'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'EventTypePatch'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypePatch'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypePatch'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypePatch')
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
