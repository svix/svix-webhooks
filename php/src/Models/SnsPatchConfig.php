<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SnsPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $topicArn = null,
        public readonly ?string $region = null,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnsPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            topicArn: null,
            region: null,
            accessKeyId: null,
            secretAccessKey: null,
            endpointUrl: null,
            setFields: []
        );
    }

    public function withTopicArn(?string $topicArn): self
    {
        $setFields = $this->setFields;
        $setFields['topicArn'] = true;

        return new self(
            topicArn: $topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $secretAccessKey,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            endpointUrl: $endpointUrl,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->topicArn) {
            $data['topicArn'] = $this->topicArn;
        }
        if (null !== $this->region) {
            $data['region'] = $this->region;
        }
        if (null !== $this->accessKeyId) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (null !== $this->secretAccessKey) {
            $data['secretAccessKey'] = $this->secretAccessKey;
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
            topicArn: \Svix\Utils::deserializeString($data, 'topicArn', false, 'SnsPatchConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'SnsPatchConfig'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SnsPatchConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'SnsPatchConfig'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SnsPatchConfig')
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
