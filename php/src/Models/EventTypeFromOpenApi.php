<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeFromOpenApi implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param string|null       $groupName    The event type group's name
     * @param string            $name         The event type's name
     */
    private function __construct(
        public readonly bool $deprecated,
        public readonly string $description,
        public readonly string $name,
        public readonly ?string $featureFlag = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $groupName = null,
        public readonly ?array $schemas = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeFromOpenApi with required fields.
     */
    public static function create(
        bool $deprecated,
        string $description,
        string $name,
    ): self {
        return new self(
            deprecated: $deprecated,
            description: $description,
            featureFlag: null,
            featureFlags: null,
            groupName: null,
            name: $name,
            schemas: null,
            setFields: ['deprecated' => true, 'description' => true, 'name' => true]
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withGroupName(?string $groupName): self
    {
        $setFields = $this->setFields;
        $setFields['groupName'] = true;

        return new self(
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $groupName,
            name: $this->name,
            schemas: $this->schemas,
            setFields: $setFields
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            deprecated: $this->deprecated,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            groupName: $this->groupName,
            name: $this->name,
            schemas: $schemas,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'deprecated' => $this->deprecated,
            'description' => $this->description,
            'name' => $this->name];

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
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'EventTypeFromOpenApi'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EventTypeFromOpenApi'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'EventTypeFromOpenApi'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypeFromOpenApi'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypeFromOpenApi'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'EventTypeFromOpenApi'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypeFromOpenApi')
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
