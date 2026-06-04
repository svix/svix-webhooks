<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventBridgeConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $detailType   Free-form string, with a maximum of 128 characters
     * @param string      $eventBusName The name or ARN of the event bus to receive the event
     */
    private function __construct(
        public readonly string $accessKeyId,
        public readonly string $eventBusName,
        public readonly string $region,
        public readonly string $secretAccessKey,
        public readonly ?string $detailType = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventBridgeConfig with required fields.
     */
    public static function create(
        string $accessKeyId,
        string $eventBusName,
        string $region,
        string $secretAccessKey,
    ): self {
        return new self(
            accessKeyId: $accessKeyId,
            detailType: null,
            eventBusName: $eventBusName,
            region: $region,
            secretAccessKey: $secretAccessKey,
            setFields: ['accessKeyId' => true, 'eventBusName' => true, 'region' => true, 'secretAccessKey' => true]
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

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKeyId' => $this->accessKeyId,
            'eventBusName' => $this->eventBusName,
            'region' => $this->region,
            'secretAccessKey' => $this->secretAccessKey];

        if (null !== $this->detailType) {
            $data['detailType'] = $this->detailType;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'EventBridgeConfig'),
            detailType: \Svix\Utils::deserializeString($data, 'detailType', false, 'EventBridgeConfig'),
            eventBusName: \Svix\Utils::deserializeString($data, 'eventBusName', true, 'EventBridgeConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'EventBridgeConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'EventBridgeConfig')
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
