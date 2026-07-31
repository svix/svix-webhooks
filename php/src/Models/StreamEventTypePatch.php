<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamEventTypePatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly ?string $description = null,
        public readonly ?array $featureFlags = null,
        public readonly ?bool $deprecated = null,
        public readonly ?bool $archived = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamEventTypePatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            description: null,
            featureFlags: null,
            deprecated: null,
            archived: null,
            setFields: []
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            featureFlags: $this->featureFlags,
            deprecated: $this->deprecated,
            archived: $this->archived,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            description: $this->description,
            featureFlags: $featureFlags,
            deprecated: $this->deprecated,
            archived: $this->archived,
            setFields: $setFields
        );
    }

    public function withDeprecated(?bool $deprecated): self
    {
        $setFields = $this->setFields;
        $setFields['deprecated'] = true;

        return new self(
            description: $this->description,
            featureFlags: $this->featureFlags,
            deprecated: $deprecated,
            archived: $this->archived,
            setFields: $setFields
        );
    }

    public function withArchived(?bool $archived): self
    {
        $setFields = $this->setFields;
        $setFields['archived'] = true;

        return new self(
            description: $this->description,
            featureFlags: $this->featureFlags,
            deprecated: $this->deprecated,
            archived: $archived,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['description'])) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (null !== $this->deprecated) {
            $data['deprecated'] = $this->deprecated;
        }
        if (null !== $this->archived) {
            $data['archived'] = $this->archived;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            description: \Svix\Utils::deserializeString($data, 'description', false, 'StreamEventTypePatch'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'StreamEventTypePatch'),
            deprecated: \Svix\Utils::deserializeBool($data, 'deprecated', false, 'StreamEventTypePatch'),
            archived: \Svix\Utils::deserializeBool($data, 'archived', false, 'StreamEventTypePatch')
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
