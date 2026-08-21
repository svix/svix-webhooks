<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for an SQS sink. */
class SqsConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $region The region of the SQS queue.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields in the future.
     * @param string|null $accessKeyId Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $secretAccessKey Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     */
    private function __construct(
        public readonly string $queueUrl,
        public readonly ?string $region = null,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SqsConfigIn with required fields.
     */
    public static function create(
        string $queueUrl,
    ): self {
        return new self(
            queueUrl: $queueUrl,
            region: null,
            accessKeyId: null,
            secretAccessKey: null,
            endpointUrl: null,
            setFields: ['queueUrl' => true]
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            queueUrl: $this->queueUrl,
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
            queueUrl: $this->queueUrl,
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
            queueUrl: $this->queueUrl,
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
            'queueUrl' => $this->queueUrl];

        if (isset($this->setFields['region'])) {
            $data['region'] = $this->region;
        }
        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['secretAccessKey'])) {
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
            queueUrl: \Svix\Utils::getValFromJson($data, 'queueUrl', true, 'SqsConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'SqsConfigIn'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SqsConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'SqsConfigIn'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SqsConfigIn')
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
