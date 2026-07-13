<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $id                the Connector's ID
     * @param string            $orgId             the Environment's ID
     * @param string|null       $uid               the Connector's UID
     * @param list<string>|null $allowedEventTypes
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly string $id,
        public readonly string $orgId,
        public readonly ConnectorKind $kind,
        public readonly string $name,
        public readonly string $description,
        public readonly string $instructions,
        public readonly string $transformation,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly \DateTimeImmutable $transformationUpdatedAt,
        public readonly ConnectorProduct $productType,
        public readonly ?string $uid = null,
        public readonly ?string $logo = null,
        public readonly ?array $allowedEventTypes = null,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ConnectorOut with required fields.
     */
    public static function create(
        string $id,
        string $orgId,
        ConnectorKind $kind,
        string $name,
        string $description,
        string $instructions,
        string $transformation,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        \DateTimeImmutable $transformationUpdatedAt,
        ConnectorProduct $productType,
    ): self {
        return new self(
            id: $id,
            orgId: $orgId,
            uid: null,
            kind: $kind,
            name: $name,
            logo: null,
            description: $description,
            instructions: $instructions,
            allowedEventTypes: null,
            transformation: $transformation,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            transformationUpdatedAt: $transformationUpdatedAt,
            featureFlags: null,
            productType: $productType,
            setFields: ['id' => true, 'orgId' => true, 'kind' => true, 'name' => true, 'description' => true, 'instructions' => true, 'transformation' => true, 'createdAt' => true, 'updatedAt' => true, 'transformationUpdatedAt' => true, 'productType' => true]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            id: $this->id,
            orgId: $this->orgId,
            uid: $uid,
            kind: $this->kind,
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            transformationUpdatedAt: $this->transformationUpdatedAt,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withLogo(?string $logo): self
    {
        $setFields = $this->setFields;
        $setFields['logo'] = true;

        return new self(
            id: $this->id,
            orgId: $this->orgId,
            uid: $this->uid,
            kind: $this->kind,
            name: $this->name,
            logo: $logo,
            description: $this->description,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            transformationUpdatedAt: $this->transformationUpdatedAt,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withAllowedEventTypes(?array $allowedEventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['allowedEventTypes'] = true;

        return new self(
            id: $this->id,
            orgId: $this->orgId,
            uid: $this->uid,
            kind: $this->kind,
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            instructions: $this->instructions,
            allowedEventTypes: $allowedEventTypes,
            transformation: $this->transformation,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            transformationUpdatedAt: $this->transformationUpdatedAt,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            id: $this->id,
            orgId: $this->orgId,
            uid: $this->uid,
            kind: $this->kind,
            name: $this->name,
            logo: $this->logo,
            description: $this->description,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            transformationUpdatedAt: $this->transformationUpdatedAt,
            featureFlags: $featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'orgId' => $this->orgId,
            'kind' => $this->kind,
            'name' => $this->name,
            'description' => $this->description,
            'instructions' => $this->instructions,
            'transformation' => $this->transformation,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'transformationUpdatedAt' => $this->transformationUpdatedAt->format('c'),
            'productType' => $this->productType];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['logo'])) {
            $data['logo'] = $this->logo;
        }
        if (isset($this->setFields['allowedEventTypes'])) {
            $data['allowedEventTypes'] = $this->allowedEventTypes;
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
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ConnectorOut'),
            orgId: \Svix\Utils::deserializeString($data, 'orgId', true, 'ConnectorOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ConnectorOut'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', true, 'ConnectorOut', [ConnectorKind::class, 'fromMixed']),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorOut'),
            logo: \Svix\Utils::getValFromJson($data, 'logo', false, 'ConnectorOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'ConnectorOut'),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', true, 'ConnectorOut'),
            allowedEventTypes: \Svix\Utils::getValFromJson($data, 'allowedEventTypes', false, 'ConnectorOut'),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ConnectorOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'ConnectorOut'),
            transformationUpdatedAt: \Svix\Utils::deserializeDt($data, 'transformationUpdatedAt', true, 'ConnectorOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorOut'),
            productType: \Svix\Utils::deserializeObject($data, 'productType', true, 'ConnectorOut', [ConnectorProduct::class, 'fromMixed'])
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
