<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null       $featureFlag  deprecated - prefer featureFlags instead
     * @param list<string>|null $featureFlags
     * @param list<string>|null $filterTypes
     */
    private function __construct(
        public readonly string $logo,
        public readonly string $name,
        public readonly string $transformation,
        public readonly ?string $description = null,
        public readonly ?string $featureFlag = null,
        public readonly ?array $featureFlags = null,
        public readonly ?array $filterTypes = null,
        public readonly ?string $instructions = null,
        public readonly ?string $instructionsLink = null,
        public readonly ?ConnectorKind $kind = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ConnectorIn with required fields.
     */
    public static function create(
        string $logo,
        string $name,
        string $transformation,
    ): self {
        return new self(
            description: null,
            featureFlag: null,
            featureFlags: null,
            filterTypes: null,
            instructions: null,
            instructionsLink: null,
            kind: null,
            logo: $logo,
            name: $name,
            transformation: $transformation,
            setFields: ['logo' => true, 'name' => true, 'transformation' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            description: $this->description,
            featureFlag: $featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $filterTypes,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withInstructions(?string $instructions): self
    {
        $setFields = $this->setFields;
        $setFields['instructions'] = true;

        return new self(
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withInstructionsLink(?string $instructionsLink): self
    {
        $setFields = $this->setFields;
        $setFields['instructionsLink'] = true;

        return new self(
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $this->instructions,
            instructionsLink: $instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function withKind(?ConnectorKind $kind): self
    {
        $setFields = $this->setFields;
        $setFields['kind'] = true;

        return new self(
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $kind,
            logo: $this->logo,
            name: $this->name,
            transformation: $this->transformation,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'logo' => $this->logo,
            'name' => $this->name,
            'transformation' => $this->transformation];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['featureFlag'])) {
            $data['featureFlag'] = $this->featureFlag;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (null !== $this->instructions) {
            $data['instructions'] = $this->instructions;
        }
        if (isset($this->setFields['instructionsLink'])) {
            $data['instructionsLink'] = $this->instructionsLink;
        }
        if (null !== $this->kind) {
            $data['kind'] = $this->kind;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            description: \Svix\Utils::deserializeString($data, 'description', false, 'ConnectorIn'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'ConnectorIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorIn'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'ConnectorIn'),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', false, 'ConnectorIn'),
            instructionsLink: \Svix\Utils::getValFromJson($data, 'instructionsLink', false, 'ConnectorIn'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', false, 'ConnectorIn', [ConnectorKind::class, 'fromMixed']),
            logo: \Svix\Utils::getValFromJson($data, 'logo', true, 'ConnectorIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorIn'),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorIn')
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
