<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IntegrationOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags the set of feature flags the integration has access to
     * @param string            $id           the Integration's ID
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly string $id,
        public readonly string $name,
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
        \DateTimeImmutable $createdAt,
        string $id,
        string $name,
        \DateTimeImmutable $updatedAt,
    ): self {
        return new self(
            createdAt: $createdAt,
            featureFlags: null,
            id: $id,
            name: $name,
            updatedAt: $updatedAt,
            setFields: ['createdAt' => true, 'id' => true, 'name' => true, 'updatedAt' => true]
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            createdAt: $this->createdAt,
            featureFlags: $featureFlags,
            id: $this->id,
            name: $this->name,
            updatedAt: $this->updatedAt,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['createdAt' => $this->createdAt->format('c'),
            'id' => $this->id,
            'name' => $this->name,
            'updatedAt' => $this->updatedAt->format('c')];

        if (null !== $this->featureFlags) {
            $data['featureFlags'] = $this->featureFlags;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'IntegrationOut'),
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'IntegrationOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'IntegrationOut'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'IntegrationOut'),
            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'IntegrationOut')
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
