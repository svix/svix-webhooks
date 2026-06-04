<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SqsPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $accessKeyId = null,
        public readonly ?string $endpointUrl = null,
        public readonly ?string $queueUrl = null,
        public readonly ?string $region = null,
        public readonly ?string $secretAccessKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SqsPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            accessKeyId: null,
            endpointUrl: null,
            queueUrl: null,
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
            endpointUrl: $this->endpointUrl,
            queueUrl: $this->queueUrl,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            endpointUrl: $endpointUrl,
            queueUrl: $this->queueUrl,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withQueueUrl(?string $queueUrl): self
    {
        $setFields = $this->setFields;
        $setFields['queueUrl'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            endpointUrl: $this->endpointUrl,
            queueUrl: $queueUrl,
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
            endpointUrl: $this->endpointUrl,
            queueUrl: $this->queueUrl,
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
            endpointUrl: $this->endpointUrl,
            queueUrl: $this->queueUrl,
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
        if (isset($this->setFields['endpointUrl'])) {
            $data['endpointUrl'] = $this->endpointUrl;
        }
        if (null !== $this->queueUrl) {
            $data['queueUrl'] = $this->queueUrl;
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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SqsPatchConfig'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SqsPatchConfig'),
            queueUrl: \Svix\Utils::getValFromJson($data, 'queueUrl', false, 'SqsPatchConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'SqsPatchConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'SqsPatchConfig')
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
