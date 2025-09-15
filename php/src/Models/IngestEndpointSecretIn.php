<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestEndpointSecretIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $key The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
     * It is recommended to not set this and let the server generate the secret.
     */
    private function __construct(
        public readonly ?string $key = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointSecretIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            key: null,
            setFields: []
        );
    }

    public function withKey(?string $key): self
    {
        $setFields = $this->setFields;
        $setFields['key'] = true;

        return new self(
            key: $key,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

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
            key: \Svix\Utils::deserializeString($data, 'key', false, 'IngestEndpointSecretIn')
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
