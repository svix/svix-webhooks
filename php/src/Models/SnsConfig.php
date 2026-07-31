<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a SNS sink. */
class SnsConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $topicArn,
        public readonly string $region,
        public readonly string $accessKeyId,
        public readonly string $secretAccessKey,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnsConfig with required fields.
     */
    public static function create(
        string $topicArn,
        string $region,
        string $accessKeyId,
        string $secretAccessKey,
    ): self {
        return new self(
            topicArn: $topicArn,
            region: $region,
            accessKeyId: $accessKeyId,
            secretAccessKey: $secretAccessKey,
            endpointUrl: null,
            setFields: ['topicArn' => true, 'region' => true, 'accessKeyId' => true, 'secretAccessKey' => true]
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
            'topicArn' => $this->topicArn,
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
            topicArn: \Svix\Utils::deserializeString($data, 'topicArn', true, 'SnsConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'SnsConfig'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'SnsConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'SnsConfig'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SnsConfig')
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
