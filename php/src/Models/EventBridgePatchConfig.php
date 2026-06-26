<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventBridgePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $accessKeyId = null,
        public readonly ?string $detailType = null,
        public readonly ?string $eventBusName = null,
        public readonly ?string $region = null,
        public readonly ?string $secretAccessKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventBridgePatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            accessKeyId: null,
            detailType: null,
            eventBusName: null,
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
            detailType: $this->detailType,
            eventBusName: $this->eventBusName,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withDetailType(?string $detailType): self
    {
        $setFields = $this->setFields;
        $setFields['detailType'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            detailType: $detailType,
            eventBusName: $this->eventBusName,
            region: $this->region,
            secretAccessKey: $this->secretAccessKey,
            setFields: $setFields
        );
    }

    public function withEventBusName(?string $eventBusName): self
    {
        $setFields = $this->setFields;
        $setFields['eventBusName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            detailType: $this->detailType,
            eventBusName: $eventBusName,
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
            detailType: $this->detailType,
            eventBusName: $this->eventBusName,
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
            detailType: $this->detailType,
            eventBusName: $this->eventBusName,
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
        if (null !== $this->detailType) {
            $data['detailType'] = $this->detailType;
        }
        if (null !== $this->eventBusName) {
            $data['eventBusName'] = $this->eventBusName;
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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'EventBridgePatchConfig'),
            detailType: \Svix\Utils::deserializeString($data, 'detailType', false, 'EventBridgePatchConfig'),
            eventBusName: \Svix\Utils::deserializeString($data, 'eventBusName', false, 'EventBridgePatchConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'EventBridgePatchConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'EventBridgePatchConfig')
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
