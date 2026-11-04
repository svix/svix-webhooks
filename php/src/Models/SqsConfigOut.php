<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SqsConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $queueUrl,
        public readonly string $region,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
        public readonly ?string $endpointUrl = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SqsConfigOut with required fields.
     */
    public static function create(
        string $queueUrl,
        string $region,
    ): self {
        return new self(
            queueUrl: $queueUrl,
            region: $region,
            accessKeyId: null,
            roleArn: null,
            externalId: null,
            endpointUrl: null,
            setFields: ['queueUrl' => true, 'region' => true]
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
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withRoleArn(?string $roleArn): self
    {
        $setFields = $this->setFields;
        $setFields['roleArn'] = true;

        return new self(
            queueUrl: $this->queueUrl,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            roleArn: $roleArn,
            externalId: $this->externalId,
            endpointUrl: $this->endpointUrl,
            setFields: $setFields
        );
    }

    public function withExternalId(?string $externalId): self
    {
        $setFields = $this->setFields;
        $setFields['externalId'] = true;

        return new self(
            queueUrl: $this->queueUrl,
            region: $this->region,
            accessKeyId: $this->accessKeyId,
            roleArn: $this->roleArn,
            externalId: $externalId,
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
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            endpointUrl: $endpointUrl,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'queueUrl' => $this->queueUrl,
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
            queueUrl: \Svix\Utils::getValFromJson($data, 'queueUrl', true, 'SqsConfigOut'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'SqsConfigOut'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'SqsConfigOut'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'SqsConfigOut'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'SqsConfigOut'),
            endpointUrl: \Svix\Utils::getValFromJson($data, 'endpointUrl', false, 'SqsConfigOut')
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
