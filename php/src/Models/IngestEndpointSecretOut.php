<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class IngestEndpointSecretOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $key The endpoint's verification secret.
     *
     * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
     * It is recommended to not set this and let the server generate the secret.
     */
    private function __construct(
        public readonly string $key,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointSecretOut with required fields.
     */
    public static function create(
        string $key,
    ): self {
        return new self(
            key: $key,
            setFields: ['key' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['key' => $this->key];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            key: \Svix\Utils::deserializeString($data, 'key', true, 'IngestEndpointSecretOut')
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
