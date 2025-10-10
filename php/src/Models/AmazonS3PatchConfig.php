<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class AmazonS3PatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $accessKeyId = null,
        public readonly ?string $bucket = null,
        public readonly ?string $region = null,
        public readonly ?string $secretAccessKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of AmazonS3PatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            accessKeyId: null,
            bucket: null,
            region: null,
            secretAccessKey: null,
            setFields: []
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            accessKeyId: $accessKeyId,
            bucket: $this->bucket,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withBucket(?string $bucket): self
    {
        $setFields = $this->setFields;
        $setFields['bucket'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            bucket: $bucket,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            bucket: $this->bucket,
            region: $region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            bucket: $this->bucket,
            region: $this->region,
            secretAccessKey: $secretAccessKey,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->accessKeyId) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (null !== $this->bucket) {
            $data['bucket'] = $this->bucket;
        }
        if (null !== $this->region) {
            $data['region'] = $this->region;
        }
        if (null !== $this->secretAccessKey) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'AmazonS3PatchConfig'),
            bucket: \Svix\Utils::deserializeString($data, 'bucket', false, 'AmazonS3PatchConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'AmazonS3PatchConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'AmazonS3PatchConfig')
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
