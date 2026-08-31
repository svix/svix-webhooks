<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3ConfigPatch implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $bucket = null,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
        public readonly ?string $region = null,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of S3ConfigPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            bucket: null,
            accessKeyId: null,
            secretAccessKey: null,
            roleArn: null,
            externalId: null,
            region: null,
            endpointUrl: null,
            setFields: []
        );
    }

    public function withBucket(?string $bucket): self
    {
        $setFields = $this->setFields;
        $setFields['bucket'] = true;

        return new self(
            bucket: $bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withRoleArn(?string $roleArn): self
    {
        $setFields = $this->setFields;
        $setFields['roleArn'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $roleArn,
            externalId: $this->externalId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withExternalId(?string $externalId): self
    {
        $setFields = $this->setFields;
        $setFields['externalId'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $externalId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $region,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $this->region,
            endpointUrl: $endpointUrl,
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
        if (null !== $this->accessKeyId) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (null !== $this->secretAccessKey) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (null !== $this->roleArn) {
            $data['roleArn'] = $this->roleArn;
        }
        if (null !== $this->externalId) {
            $data['externalId'] = $this->externalId;
        }
        if (null !== $this->region) {
            $data['region'] = $this->region;
        }
        if (null !== $this->endpointUrl) {
            $data['endpointUrl'] = $this->endpointUrl;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            bucket: \Svix\Utils::deserializeString($data, 'bucket', false, 'S3ConfigPatch'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'S3ConfigPatch'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'S3ConfigPatch'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'S3ConfigPatch'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'S3ConfigPatch'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'S3ConfigPatch'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'S3ConfigPatch')
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
