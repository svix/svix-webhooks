<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $allowedEventTypes
     * @param list<string>|null $featureFlags
     * @param string|null       $uid               the Connector's UID
     */
    private function __construct(
        public readonly string $name,
        public readonly string $transformation,
        public readonly ?array $allowedEventTypes = null,
        public readonly ?string $description = null,
        public readonly ?array $featureFlags = null,
        public readonly ?string $instructions = null,
        public readonly ?ConnectorKind $kind = null,
        public readonly ?string $logo = null,
        public readonly ?ConnectorProduct $productType = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ConnectorIn with required fields.
     */
    public static function create(
        string $name,
        string $transformation,
    ): self {
        return new self(
            allowedEventTypes: null,
            description: null,
            featureFlags: null,
            instructions: null,
            kind: null,
            logo: null,
            name: $name,
            productType: null,
            transformation: $transformation,
            uid: null,
            setFields: ['name' => true, 'transformation' => true]
        );
    }

    public function withAllowedEventTypes(?array $allowedEventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['allowedEventTypes'] = true;

        return new self(
            allowedEventTypes: $allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withInstructions(?string $instructions): self
    {
        $setFields = $this->setFields;
        $setFields['instructions'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withKind(?ConnectorKind $kind): self
    {
        $setFields = $this->setFields;
        $setFields['kind'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withLogo(?string $logo): self
    {
        $setFields = $this->setFields;
        $setFields['logo'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withProductType(?ConnectorProduct $productType): self
    {
        $setFields = $this->setFields;
        $setFields['productType'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $productType,
            transformation: $this->transformation,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            allowedEventTypes: $this->allowedEventTypes,
            description: $this->description,
            featureFlags: $this->featureFlags,
            instructions: $this->instructions,
            kind: $this->kind,
            logo: $this->logo,
            name: $this->name,
            productType: $this->productType,
            transformation: $this->transformation,
            uid: $uid,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'transformation' => $this->transformation];

        if (isset($this->setFields['allowedEventTypes'])) {
            $data['allowedEventTypes'] = $this->allowedEventTypes;
        }
        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (null !== $this->instructions) {
            $data['instructions'] = $this->instructions;
        }
        if (null !== $this->kind) {
            $data['kind'] = $this->kind;
        }
        if (isset($this->setFields['logo'])) {
            $data['logo'] = $this->logo;
        }
        if (isset($this->setFields['productType'])) {
            $data['productType'] = $this->productType;
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
            allowedEventTypes: \Svix\Utils::getValFromJson($data, 'allowedEventTypes', false, 'ConnectorIn'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'ConnectorIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorIn'),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', false, 'ConnectorIn'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', false, 'ConnectorIn', [ConnectorKind::class, 'fromMixed']),
            logo: \Svix\Utils::getValFromJson($data, 'logo', false, 'ConnectorIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorIn'),
            productType: \Svix\Utils::deserializeObject($data, 'productType', false, 'ConnectorIn', [ConnectorProduct::class, 'fromMixed']),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ConnectorIn')
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
