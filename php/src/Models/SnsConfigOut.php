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
        public readonly ?string $accessKeyId = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
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
    ): self {
        return new self(
            topicArn: $topicArn,
            region: $region,
            accessKeyId: null,
            roleArn: null,
            externalId: null,
            setFields: ['topicArn' => true, 'region' => true]
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
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withRoleArn(?string $roleArn): self
    {
        $setFields = $this->setFields;
        $setFields['roleArn'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            roleArn: $roleArn,
            externalId: $this->externalId,
            setFields: $setFields
        );
    }

    public function withExternalId(?string $externalId): self
    {
        $setFields = $this->setFields;
        $setFields['externalId'] = true;

        return new self(
            topicArn: $this->topicArn,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            roleArn: $this->roleArn,
            externalId: $externalId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'topicArn' => $this->topicArn,
            'region' => $this->region];

        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['roleArn'])) {
            $data['roleArn'] = $this->roleArn;
        }
        if (isset($this->setFields['externalId'])) {
            $data['externalId'] = $this->externalId;
        }

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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SnsConfigOut'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'SnsConfigOut'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'SnsConfigOut')
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
