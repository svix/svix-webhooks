<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for an SQS sink. */
class SqsConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $queueUrl,
        public readonly string $region,
        public readonly string $accessKeyId,
        public readonly string $secretAccessKey,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SqsConfig with required fields.
     */
    public static function create(
        string $queueUrl,
        string $region,
        string $accessKeyId,
        string $secretAccessKey,
    ): self {
        return new self(
            queueUrl: $queueUrl,
            region: $region,
            accessKeyId: $accessKeyId,
            secretAccessKey: $secretAccessKey,
            endpointUrl: null,
            setFields: ['queueUrl' => true, 'region' => true, 'accessKeyId' => true, 'secretAccessKey' => true]
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            queueUrl: $this->queueUrl,
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
            'queueUrl' => $this->queueUrl,
            'region' => $this->region,
            'accessKeyId' => $this->accessKeyId,
            'secretAccessKey' => $this->secretAccessKey];

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
            queueUrl: \Svix\Utils::getValFromJson($data, 'queueUrl', true, 'SqsConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'SqsConfig'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'SqsConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'SqsConfig'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SqsConfig')
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
