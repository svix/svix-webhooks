<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3ConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $accessKeyId Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $secretAccessKey Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $region The region of the EventBridge bus.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields in the future.
     */
    private function __construct(
        public readonly string $bucket,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $region = null,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of S3ConfigIn with required fields.
     */
    public static function create(
        string $bucket,
    ): self {
        return new self(
            bucket: $bucket,
            accessKeyId: null,
            secretAccessKey: null,
            region: null,
            endpointUrl: null,
            setFields: ['bucket' => true]
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
            region: $this->region,
            endpointUrl: $endpointUrl,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'bucket' => $this->bucket];

        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['secretAccessKey'])) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (isset($this->setFields['region'])) {
            $data['region'] = $this->region;
        }
        if (isset($this->setFields['endpointUrl'])) {
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
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'S3ConfigIn'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'S3ConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'S3ConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'S3ConfigIn'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'S3ConfigIn')
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
