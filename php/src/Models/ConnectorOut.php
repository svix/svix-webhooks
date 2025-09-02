<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags
     * @param list<string>|null $filterTypes
     * @param string            $id           the Connector's ID
     * @param string            $orgId        the Environment's ID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $description,
        public readonly string $id,
        public readonly string $instructions,
        public readonly ConnectorKind $kind,
        public readonly string $logo,
        public readonly string $name,
        public readonly string $orgId,
        public readonly string $transformation,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?string $featureFlag = null,
        public readonly ?array $featureFlags = null,
        public readonly ?array $filterTypes = null,
        public readonly ?string $instructionsLink = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ConnectorOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        string $description,
        string $id,
        string $instructions,
        ConnectorKind $kind,
        string $logo,
        string $name,
        string $orgId,
        string $transformation,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            createdAt: $createdAt,
            description: $description,
            featureFlag: null,
            featureFlags: null,
            filterTypes: null,
            id: $id,
            instructions: $instructions,
            instructionsLink: null,
            kind: $kind,
            logo: $logo,
            name: $name,
            orgId: $orgId,
            transformation: $transformation,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'description' => true, 'id' => true, 'instructions' => true, 'kind' => true, 'logo' => true, 'name' => true, 'orgId' => true, 'transformation' => true, 'updatedAt' => true]
        );
    }

    public function withFeatureFlag(?string $featureFlag): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlag'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlag: $featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            id: $this->id,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            transformation: $this->transformation,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $featureFlags,
            filterTypes: $this->filterTypes,
            id: $this->id,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            transformation: $this->transformation,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFilterTypes(?array $filterTypes): self
    {
        $setFields = $this->setFields;
        $setFields['filterTypes'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $filterTypes,
            id: $this->id,
            instructions: $this->instructions,
            instructionsLink: $this->instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            transformation: $this->transformation,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withInstructionsLink(?string $instructionsLink): self
    {
        $setFields = $this->setFields;
        $setFields['instructionsLink'] = true;

        return new self(
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlag: $this->featureFlag,
            featureFlags: $this->featureFlags,
            filterTypes: $this->filterTypes,
            id: $this->id,
            instructions: $this->instructions,
            instructionsLink: $instructionsLink,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            transformation: $this->transformation,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'description' => $this->description,
            'id' => $this->id,
            'instructions' => $this->instructions,
            'kind' => $this->kind,
            'logo' => $this->logo,
            'name' => $this->name,
            'orgId' => $this->orgId,
            'transformation' => $this->transformation,
            'updatedAt' => $this->updatedAt->format('c')];

        if (isset($this->setFields['featureFlag'])) {
            $data['featureFlag'] = $this->featureFlag;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['filterTypes'])) {
            $data['filterTypes'] = $this->filterTypes;
        }
        if (isset($this->setFields['instructionsLink'])) {
            $data['instructionsLink'] = $this->instructionsLink;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ConnectorOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'ConnectorOut'),
            featureFlag: \Svix\Utils::deserializeString($data, 'featureFlag', false, 'ConnectorOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorOut'),
            filterTypes: \Svix\Utils::getValFromJson($data, 'filterTypes', false, 'ConnectorOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ConnectorOut'),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', true, 'ConnectorOut'),
            instructionsLink: \Svix\Utils::getValFromJson($data, 'instructionsLink', false, 'ConnectorOut'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', true, 'ConnectorOut', [ConnectorKind::class, 'fromMixed']),
            logo: \Svix\Utils::getValFromJson($data, 'logo', true, 'ConnectorOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorOut'),
            orgId: \Svix\Utils::deserializeString($data, 'orgId', true, 'ConnectorOut'),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'ConnectorOut')
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
