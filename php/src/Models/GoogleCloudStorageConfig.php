<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * Configuration for a Google Cloud Storage sink.
 *
 * Write stream events into the named bucket using the supplied Google Cloud credentials.
 */
class GoogleCloudStorageConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $credentials google Cloud Credentials JSON Object as a string
     */
    private function __construct(
        public readonly string $bucket,
        public readonly string $credentials,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudStorageConfig with required fields.
     */
    public static function create(
        string $bucket,
        string $credentials,
    ): self {
        return new self(
            bucket: $bucket,
            credentials: $credentials,
            setFields: ['bucket' => true, 'credentials' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'bucket' => $this->bucket,
            'credentials' => $this->credentials];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'GoogleCloudStorageConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', true, 'GoogleCloudStorageConfig')
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
