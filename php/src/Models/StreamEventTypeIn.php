<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamEventTypeIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param string            $name         The event type's name
     */
    private function __construct(
        public readonly string $name,
        public readonly ?bool $archived = null,
        public readonly ?bool $deprecated = null,
        public readonly ?string $description = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamEventTypeIn with required fields.
     */
    public static function create(
        string $name,
    ): self {
        return new self(
            archived: null,
            deprecated: null,
            description: null,
            featureFlags: null,
            name: $name,
            setFields: ['name' => true]
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
            featureFlags: $this->featureFlags,
            name: $this->name,
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
            featureFlags: $this->featureFlags,
            name: $this->name,
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
            featureFlags: $this->featureFlags,
            name: $this->name,
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
            featureFlags: $featureFlags,
            name: $this->name,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name];

        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }
        if (null !== $this->deprecated) {
            $data['deprecated'] = $this->deprecated;
        }
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
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'StreamEventTypeIn'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', false, 'StreamEventTypeIn'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'StreamEventTypeIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'StreamEventTypeIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'StreamEventTypeIn')
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
