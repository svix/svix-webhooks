<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventBridgeConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $eventBusName,
        public readonly string $detailType,
        public readonly string $region,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $roleArn = null,
        public readonly ?string $externalId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventBridgeConfigOut with required fields.
     */
    public static function create(
        string $eventBusName,
        string $detailType,
        string $region,
    ): self {
        return new self(
            eventBusName: $eventBusName,
            detailType: $detailType,
            accessKeyId: null,
            roleArn: null,
            externalId: null,
            region: $region,
            setFields: ['eventBusName' => true, 'detailType' => true, 'region' => true]
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $this->detailType,
            accessKeyId: $accessKeyId,
            roleArn: $this->roleArn,
            externalId: $this->externalId,
            region: $this->region,
            setFields: $setFields
        );
    }

    public function withRoleArn(?string $roleArn): self
    {
        $setFields = $this->setFields;
        $setFields['roleArn'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $this->detailType,
            accessKeyId: $this->accessKeyId,
            roleArn: $roleArn,
            externalId: $this->externalId,
            region: $this->region,
            setFields: $setFields
        );
    }

    public function withExternalId(?string $externalId): self
    {
        $setFields = $this->setFields;
        $setFields['externalId'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $this->detailType,
            accessKeyId: $this->accessKeyId,
            roleArn: $this->roleArn,
            externalId: $externalId,
            region: $this->region,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventBusName' => $this->eventBusName,
            'detailType' => $this->detailType,
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
            eventBusName: \Svix\Utils::deserializeString($data, 'eventBusName', true, 'EventBridgeConfigOut'),
            detailType: \Svix\Utils::deserializeString($data, 'detailType', true, 'EventBridgeConfigOut'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'EventBridgeConfigOut'),
            roleArn: \Svix\Utils::deserializeString($data, 'roleArn', false, 'EventBridgeConfigOut'),
            externalId: \Svix\Utils::deserializeString($data, 'externalId', false, 'EventBridgeConfigOut'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'EventBridgeConfigOut')
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
