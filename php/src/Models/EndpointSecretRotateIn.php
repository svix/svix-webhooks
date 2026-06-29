<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointSecretRotateIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $gracePeriodSeconds How long the old secret will be valid for, in seconds.
     *
     * Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours.
     * @param string|null $key The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
     * It is recommended to not set this and let the server generate the secret.
     */
    private function __construct(
        public readonly ?int $gracePeriodSeconds = null,
        public readonly ?string $key = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointSecretRotateIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            gracePeriodSeconds: null,
            key: null,
            setFields: []
        );
    }

    public function withGracePeriodSeconds(?int $gracePeriodSeconds): self
    {
        $setFields = $this->setFields;
        $setFields['gracePeriodSeconds'] = true;

        return new self(
            gracePeriodSeconds: $gracePeriodSeconds,
            key: $this->key,
            setFields: $setFields
        );
    }

    public function withKey(?string $key): self
    {
        $setFields = $this->setFields;
        $setFields['key'] = true;

        return new self(
            gracePeriodSeconds: $this->gracePeriodSeconds,
            key: $key,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['gracePeriodSeconds'])) {
            $data['gracePeriodSeconds'] = $this->gracePeriodSeconds;
        }
        if (isset($this->setFields['key'])) {
            $data['key'] = $this->key;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            gracePeriodSeconds: \Svix\Utils::deserializeInt($data, 'gracePeriodSeconds', false, 'EndpointSecretRotateIn'),
            key: \Svix\Utils::deserializeString($data, 'key', false, 'EndpointSecretRotateIn')
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
