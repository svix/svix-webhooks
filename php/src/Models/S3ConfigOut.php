<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3ConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $bucket,
        public readonly string $region,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $endpointUrl = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of S3ConfigOut with required fields.
     */
    public static function create(
        string $bucket,
        string $region,
    ): self {
        return new self(
            bucket: $bucket,
            accessKeyId: null,
            region: $region,
            endpointUrl: null,
            roleArn: null,
            externalId: null,
            setFields: ['bucket' => true, 'region' => true]
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            bucket: $this->bucket,
            accessKeyId: $accessKeyId,
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
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
            region: $this->region,
            endpointUrl: $endpointUrl,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
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
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            roleArn: $roleArn,
            externalId: $this->externalId,
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
            region: $this->region,
            endpointUrl: $this->endpointUrl,
            roleArn: $this->roleArn,
            externalId: $externalId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'bucket' => $this->bucket,
            'region' => $this->region];

        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['endpointUrl'])) {
            $data['endpointUrl'] = $this->endpointUrl;
        }
        if (isset($this->setFields['roleArn'])) {
            $data['roleArn'] = $this->roleArn;
        }
        if (isset($this->setFields['externalId'])) {
            $data['externalId'] = $this->externalId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'S3ConfigOut'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'S3ConfigOut'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'S3ConfigOut'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'S3ConfigOut'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'S3ConfigOut'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'S3ConfigOut')
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
