<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ConnectorIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null       $uid               the Connector's UID
     * @param list<string>|null $allowedEventTypes
     * @param list<string>|null $featureFlags
     */
    private function __construct(
        public readonly string $name,
        public readonly string $transformation,
        public readonly ?string $uid = null,
        public readonly ?string $logo = null,
        public readonly ?string $description = null,
        public readonly ?ConnectorKind $kind = null,
        public readonly ?string $instructions = null,
        public readonly ?array $allowedEventTypes = null,
        public readonly ?array $featureFlags = null,
        public readonly ?ConnectorProduct $productType = null,
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
            name: $name,
            uid: null,
            logo: null,
            description: null,
            kind: null,
            instructions: null,
            allowedEventTypes: null,
            transformation: $transformation,
            featureFlags: null,
            productType: null,
            setFields: ['name' => true, 'transformation' => true]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            name: $this->name,
            uid: $uid,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
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
            name: $this->name,
            uid: $this->uid,
            logo: $logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withKind(?ConnectorKind $kind): self
    {
        $setFields = $this->setFields;
        $setFields['kind'] = true;

        return new self(
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $this->description,
            kind: $kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withInstructions(?string $instructions): self
    {
        $setFields = $this->setFields;
        $setFields['instructions'] = true;

        return new self(
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
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
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $allowedEventTypes,
            transformation: $this->transformation,
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
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $featureFlags,
            productType: $this->productType,
            setFields: $setFields
        );
    }

    public function withProductType(?ConnectorProduct $productType): self
    {
        $setFields = $this->setFields;
        $setFields['productType'] = true;

        return new self(
            name: $this->name,
            uid: $this->uid,
            logo: $this->logo,
            description: $this->description,
            kind: $this->kind,
            instructions: $this->instructions,
            allowedEventTypes: $this->allowedEventTypes,
            transformation: $this->transformation,
            featureFlags: $this->featureFlags,
            productType: $productType,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'transformation' => $this->transformation];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
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
        if (isset($this->setFields['featureFlags'])) {
            $data['featureFlags'] = $this->featureFlags;
        }
        if (isset($this->setFields['productType'])) {
            $data['productType'] = $this->productType;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ConnectorIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ConnectorIn'),
            logo: \Svix\Utils::getValFromJson($data, 'logo', false, 'ConnectorIn'),
            description: \Svix\Utils::deserializeString($data, 'description', false, 'ConnectorIn'),
            kind: \Svix\Utils::deserializeObject($data, 'kind', false, 'ConnectorIn', [ConnectorKind::class, 'fromMixed']),
            instructions: \Svix\Utils::deserializeString($data, 'instructions', false, 'ConnectorIn'),
            allowedEventTypes: \Svix\Utils::getValFromJson($data, 'allowedEventTypes', false, 'ConnectorIn'),
            transformation: \Svix\Utils::deserializeString($data, 'transformation', true, 'ConnectorIn'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'ConnectorIn'),
            productType: \Svix\Utils::deserializeObject($data, 'productType', false, 'ConnectorIn', [ConnectorProduct::class, 'fromMixed'])
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
