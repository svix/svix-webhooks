<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3Config implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $accessKeyId,
        public readonly string $bucket,
        public readonly string $region,
        public readonly string $secretAccessKey,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of S3Config with required fields.
     */
    public static function create(
        string $accessKeyId,
        string $bucket,
        string $region,
        string $secretAccessKey,
    ): self {
        return new self(
            accessKeyId: $accessKeyId,
            bucket: $bucket,
            region: $region,
            secretAccessKey: $secretAccessKey,
            setFields: ['accessKeyId' => true, 'bucket' => true, 'region' => true, 'secretAccessKey' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKeyId' => $this->accessKeyId,
            'bucket' => $this->bucket,
            'region' => $this->region,
            'secretAccessKey' => $this->secretAccessKey];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'S3Config'),
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'S3Config'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'S3Config'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'S3Config')
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
