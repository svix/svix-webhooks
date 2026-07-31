<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IntegrationOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string            $id           the Integration's ID
     * @param list<string>|null $featureFlags the set of feature flags the integration has access to
     */
    private function __construct(
        public readonly string $name,
        public readonly string $id,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IntegrationOut with required fields.
     */
    public static function create(
        string $name,
        string $id,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            name: $name,
            id: $id,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            featureFlags: null,
            setFields: ['name' => true, 'id' => true, 'createdAt' => true, 'updatedAt' => true]
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            name: $this->name,
            id: $this->id,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name,
            'id' => $this->id,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c')];

        if (null !== $this->featureFlags) {
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
            name: \Svix\Utils::deserializeString($data, 'name', true, 'IntegrationOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'IntegrationOut'),
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'IntegrationOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'IntegrationOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'IntegrationOut')
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
