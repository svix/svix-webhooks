<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class S3ConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $bucket,
        public readonly string $accessKeyId,
        public readonly string $region,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of S3ConfigOut with required fields.
     */
    public static function create(
        string $bucket,
        string $accessKeyId,
        string $region,
    ): self {
        return new self(
            bucket: $bucket,
            accessKeyId: $accessKeyId,
            region: $region,
            endpointUrl: null,
            setFields: ['bucket' => true, 'accessKeyId' => true, 'region' => true]
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
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'bucket' => $this->bucket,
            'accessKeyId' => $this->accessKeyId,
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
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'S3ConfigOut'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'S3ConfigOut'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'S3ConfigOut'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'S3ConfigOut')
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
