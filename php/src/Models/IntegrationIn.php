<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IntegrationIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags the set of feature flags the integration will have access to
     */
    private function __construct(
        public readonly string $name,
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IntegrationIn with required fields.
     */
    public static function create(
        string $name,
    ): self {
        return new self(
            featureFlags: null,
            name: $name,
            setFields: ['name' => true]
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            featureFlags: $featureFlags,
            name: $this->name,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name];

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
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'IntegrationIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'IntegrationIn')
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
