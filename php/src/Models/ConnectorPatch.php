<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $allowedEventTypes
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly ?string $name = null,
        public readonly ?string $logo = null,
        public readonly ?string $description = null,
        public readonly ?ConnectorKind $kind = null,
        public readonly ?string $instructions = null,
        public readonly ?array $allowedEventTypes = null,
        public readonly ?string $transformation = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ConnectorPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            name: null,
            logo: null,
            description: null,
            kind: null,
            instructions: null,
            allowedEventTypes: null,
            transformation: null,
            featureFlags: null,
            setFields: []
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            name: $name,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withLogo(?string $logo): self
    {
        $setFields = $this->setFields;
        $setFields['logo'] = true;

        return new self(
            name: $this->name,
            logo: $logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            name: $this->name,
            logo: $this->logo,
            description: $description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withKind(?ConnectorKind $kind): self
    {
        $setFields = $this->setFields;
        $setFields['kind'] = true;

        return new self(
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            kind: $kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withInstructions(?string $instructions): self
    {
        $setFields = $this->setFields;
        $setFields['instructions'] = true;

        return new self(
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withAllowedEventTypes(?array $allowedEventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['allowedEventTypes'] = true;

        return new self(
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            setFields: $setFields
        );
    }

    public function withTransformation(?string $transformation): self
    {
        $setFields = $this->setFields;
        $setFields['transformation'] = true;

        return new self(
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $transformation,
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
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->name) {
            $data['name'] = $this->name;
        }
        if (isset($this->setFields['logo'])) {
            $data['logo'] = $this->logo;
        }
        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (null !== $this->kind) {
            $data['kind'] = $this->kind;
        }
        if (null !== $this->instructions) {
            $data['instructions'] = $this->instructions;
        }
        if (isset($this->setFields['allowedEventTypes'])) {
            $data['allowedEventTypes'] = $this->allowedEventTypes;
        }
        if (null !== $this->transformation) {
            $data['transformation'] = $this->transformation;
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
            name: \Svix\Utils::deserializeString($data, 'name', false, 'ConnectorPatch'),
            logo: \Svix\Utils::getValFromJson($data, 'logo', false, 'ConnectorPatch'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'ConnectorPatch'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', false, 'ConnectorPatch', [ConnectorKind::class, 'fromMixed']),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', false, 'ConnectorPatch'),
            allowedEventTypes: \Svix\Utils::getValFromJson($data, 'allowedEventTypes', false, 'ConnectorPatch'),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', false, 'ConnectorPatch'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorPatch')
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
