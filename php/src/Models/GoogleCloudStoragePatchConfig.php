<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class GoogleCloudStoragePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $bucket = null,
        public readonly ?string $credentials = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudStoragePatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            bucket: null,
            credentials: null,
            setFields: []
        );
    }

    public function withBucket(?string $bucket): self
    {
        $setFields = $this->setFields;
        $setFields['bucket'] = true;

        return new self(
            bucket: $bucket,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withCredentials(?string $credentials): self
    {
        $setFields = $this->setFields;
        $setFields['credentials'] = true;

        return new self(
            bucket: $this->bucket,
            credentials: $credentials,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->bucket) {
            $data['bucket'] = $this->bucket;
        }
        if (null !== $this->credentials) {
            $data['credentials'] = $this->credentials;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            bucket: \Svix\Utils::deserializeString($data, 'bucket', false, 'GoogleCloudStoragePatchConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', false, 'GoogleCloudStoragePatchConfig')
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
