<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a SNS sink. */
class SnsConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $accessKeyId,
        public readonly string $region,
        public readonly string $secretAccessKey,
        public readonly string $topicArn,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnsConfig with required fields.
     */
    public static function create(
        string $accessKeyId,
        string $region,
        string $secretAccessKey,
        string $topicArn,
    ): self {
        return new self(
            accessKeyId: $accessKeyId,
            endpointUrl: null,
            region: $region,
            secretAccessKey: $secretAccessKey,
            topicArn: $topicArn,
            setFields: ['accessKeyId' => true, 'region' => true, 'secretAccessKey' => true, 'topicArn' => true]
        );
    }

    public function withEndpointUrl(?string $endpointUrl): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUrl'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            endpointUrl: $endpointUrl,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            topicArn: $this->topicArn,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKeyId' => $this->accessKeyId,
            'region' => $this->region,
            'secretAccessKey' => $this->secretAccessKey,
            'topicArn' => $this->topicArn];

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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'SnsConfig'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SnsConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'SnsConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'SnsConfig'),
            topicArn: \Svix\Utils::deserializeString($data, 'topicArn', true, 'SnsConfig')
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
