<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventBridgeConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $eventBusName The name or ARN of the event bus to receive the event
     * @param string|null $detailType   Free-form string, with a maximum of 128 characters
     * @param string|null $accessKeyId  Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $secretAccessKey Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $region The region of the EventBridge bus.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields in the future.
     */
    private function __construct(
        public readonly string $eventBusName,
        public readonly ?string $detailType = null,
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $region = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventBridgeConfigIn with required fields.
     */
    public static function create(
        string $eventBusName,
    ): self {
        return new self(
            eventBusName: $eventBusName,
            detailType: null,
            accessKeyId: null,
            secretAccessKey: null,
            region: null,
            setFields: ['eventBusName' => true]
        );
    }

    public function withDetailType(?string $detailType): self
    {
        $setFields = $this->setFields;
        $setFields['detailType'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $detailType,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            setFields: $setFields
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
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $this->detailType,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $secretAccessKey,
            region: $this->region,
            setFields: $setFields
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            eventBusName: $this->eventBusName,
            detailType: $this->detailType,
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            region: $region,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventBusName' => $this->eventBusName];

        if (null !== $this->detailType) {
            $data['detailType'] = $this->detailType;
        }
        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['secretAccessKey'])) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (isset($this->setFields['region'])) {
            $data['region'] = $this->region;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            eventBusName: \Svix\Utils::deserializeString($data, 'eventBusName', true, 'EventBridgeConfigIn'),
            detailType: \Svix\Utils::deserializeString($data, 'detailType', false, 'EventBridgeConfigIn'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'EventBridgeConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'EventBridgeConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'EventBridgeConfigIn')
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
