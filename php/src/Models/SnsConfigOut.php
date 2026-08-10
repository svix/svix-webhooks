<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SnsConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $topicArn,
        public readonly string $region,
        public readonly string $accessKeyId,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnsConfigOut with required fields.
     */
    public static function create(
        string $topicArn,
        string $region,
        string $accessKeyId,
    ): self {
        return new self(
            topicArn: $topicArn,
            region: $region,
            accessKeyId: $accessKeyId,
            setFields: ['topicArn' => true, 'region' => true, 'accessKeyId' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'topicArn' => $this->topicArn,
            'region' => $this->region,
            'accessKeyId' => $this->accessKeyId];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            topicArn: \Svix\Utils::deserializeString($data, 'topicArn', true, 'SnsConfigOut'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'SnsConfigOut'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'SnsConfigOut')
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
