<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventBridgeConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $eventBusName The name or ARN of the event bus to receive the event
     * @param string|null $detailType   Free-form string, with a maximum of 128 characters
     */
    private function __construct(
        public readonly string $eventBusName,
        public readonly string $accessKeyId,
        public readonly string $secretAccessKey,
        public readonly string $region,
        public readonly ?string $detailType = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventBridgeConfig with required fields.
     */
    public static function create(
        string $eventBusName,
        string $accessKeyId,
        string $secretAccessKey,
        string $region,
    ): self {
        return new self(
            eventBusName: $eventBusName,
            detailType: null,
            accessKeyId: $accessKeyId,
            secretAccessKey: $secretAccessKey,
            region: $region,
            setFields: ['eventBusName' => true, 'accessKeyId' => true, 'secretAccessKey' => true, 'region' => true]
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

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventBusName' => $this->eventBusName,
            'accessKeyId' => $this->accessKeyId,
            'secretAccessKey' => $this->secretAccessKey,
            'region' => $this->region];

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
            eventBusName: \Svix\Utils::deserializeString($data, 'eventBusName', true, 'EventBridgeConfig'),
            detailType: \Svix\Utils::deserializeString($data, 'detailType', false, 'EventBridgeConfig'),
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'EventBridgeConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'EventBridgeConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'EventBridgeConfig')
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
