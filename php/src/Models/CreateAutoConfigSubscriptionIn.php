<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class CreateAutoConfigSubscriptionIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $featureFlags The set of feature flags the created token will have access to.
     *
     * When omitted or empty, the token inherits the calling token's feature flags.
     * When set, these flags are used instead. An application token may only grant a subset of its own flags.
     */
    private function __construct(
        public readonly ?array $featureFlags = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of CreateAutoConfigSubscriptionIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            featureFlags: null,
            setFields: []
        );
    }

    public function withFeatureFlags(?array $featureFlags): self
    {
        $setFields = $this->setFields;
        $setFields['featureFlags'] = true;

        return new self(
            featureFlags: $featureFlags,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

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
            featureFlags: \Svix\Utils::getValFromJson($data, 'featureFlags', false, 'CreateAutoConfigSubscriptionIn')
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
