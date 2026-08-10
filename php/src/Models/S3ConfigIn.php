<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3ConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $bucket,
        public readonly string $accessKeyId,
        public readonly string $secretAccessKey,
        public readonly string $region,
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
        string $accessKeyId,
        string $secretAccessKey,
        string $region,
    ): self {
        return new self(
            bucket: $bucket,
            accessKeyId: $accessKeyId,
            secretAccessKey: $secretAccessKey,
            region: $region,
            endpointUrl: null,
            setFields: ['bucket' => true, 'accessKeyId' => true, 'secretAccessKey' => true, 'region' => true]
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
            'bucket' => $this->bucket,
            'accessKeyId' => $this->accessKeyId,
            'secretAccessKey' => $this->secretAccessKey,
            'region' => $this->region];

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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'S3ConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'S3ConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'S3ConfigIn'),
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
