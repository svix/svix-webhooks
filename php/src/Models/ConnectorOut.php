<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $allowedEventTypes
     * @param list<string>|null $featureFlags
     * @param string            $id                the Connector's ID
     * @param string            $orgId             the Environment's ID
     * @param string|null       $uid               the Connector's UID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $description,
        public readonly string $id,
        public readonly string $instructions,
        public readonly ConnectorKind $kind,
        public readonly string $name,
        public readonly string $orgId,
        public readonly ConnectorProduct $productType,
        public readonly string $transformation,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?array $allowedEventTypes = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $logo = null,
        public readonly ?string $uid = null,
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
        string $name,
        string $orgId,
        ConnectorProduct $productType,
        string $transformation,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            allowedEventTypes: null,
            createdAt: $createdAt,
            description: $description,
            featureFlags: null,
            id: $id,
            instructions: $instructions,
            kind: $kind,
            logo: null,
            name: $name,
            orgId: $orgId,
            productType: $productType,
            transformation: $transformation,
            uid: null,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'description' => true, 'id' => true, 'instructions' => true, 'kind' => true, 'name' => true, 'orgId' => true, 'productType' => true, 'transformation' => true, 'updatedAt' => true]
        );
    }

    public function withAllowedEventTypes(?array $allowedEventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['allowedEventTypes'] = true;

        return new self(
            allowedEventTypes: $allowedEventTypes,
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlags: $this->featureFlags,
            id: $this->id,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlags: $featureFlags,
            id: $this->id,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withLogo(?string $logo): self
    {
        $setFields = $this->setFields;
        $setFields['logo'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlags: $this->featureFlags,
            id: $this->id,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $logo,
            name: $this->name,
            orgId: $this->orgId,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            createdAt: $this->createdAt,
            description: $this->description,
            featureFlags: $this->featureFlags,
            id: $this->id,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            orgId: $this->orgId,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $uid,
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
            'name' => $this->name,
            'orgId' => $this->orgId,
            'productType' => $this->productType,
            'transformation' => $this->transformation,
            'updatedAt' => $this->updatedAt->format('c')];

        if (isset($this->setFields['allowedEventTypes'])) {
            $data['allowedEventTypes'] = $this->allowedEventTypes;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['logo'])) {
            $data['logo'] = $this->logo;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            allowedEventTypes: \Svix\Utils::getValFromJson($data, 'allowedEventTypes', false, 'ConnectorOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'ConnectorOut'),
            description: \Svix\Utils::deserializeString($data, 'description', true, 'ConnectorOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'ConnectorOut'),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', true, 'ConnectorOut'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', true, 'ConnectorOut', [ConnectorKind::class, 'fromMixed']),
            logo: \Svix\Utils::getValFromJson($data, 'logo', false, 'ConnectorOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorOut'),
            orgId: \Svix\Utils::deserializeString($data, 'orgId', true, 'ConnectorOut'),
            productType: \Svix\Utils::deserializeObject($data, 'productType', true, 'ConnectorOut', [ConnectorProduct::class, 'fromMixed']),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorOut'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ConnectorOut'),
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
