<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeFromOpenApi implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $name         The event type's name
     * @param string|null       $groupName    The event type group's name
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly string $name,
        public readonly string $description,
        public readonly bool $deprecated,
        public readonly ?array $schemas = null,
        public readonly ?string $groupName = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeFromOpenApi with required fields.
     */
    public static function create(
        string $name,
        string $description,
        bool $deprecated,
    ): self {
        return new self(
            name: $name,
            description: $description,
            schemas: null,
            deprecated: $deprecated,
            groupName: null,
            featureFlags: null,
            setFields: ['name' => true, 'description' => true, 'deprecated' => true]
        );
    }

    public function withSchemas(?array $schemas): self
    {
        $setFields = $this->setFields;
        $setFields['schemas'] = true;

        return new self(
            name: $this->name,
            description: $this->description,
            schemas: $schemas,
            deprecated: $this->deprecated,
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
            schemas: $this->schemas,
            deprecated: $this->deprecated,
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
            schemas: $this->schemas,
            deprecated: $this->deprecated,
            groupName: $this->groupName,
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'description' => $this->description,
            'deprecated' => $this->deprecated];

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
            name: \Svix\Utils::deserializeString($data, 'name', true, 'EventTypeFromOpenApi'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'EventTypeFromOpenApi'),
            schemas: \Svix\Utils::getValFromJson($data, 'schemas', false, 'EventTypeFromOpenApi'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', true, 'EventTypeFromOpenApi'),
            groupName: \Svix\Utils::deserializeString($data, 'groupName', false, 'EventTypeFromOpenApi'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'EventTypeFromOpenApi')
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
