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
        public readonly string $accessKeyId,
        public readonly string $region,
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
        string $accessKeyId,
        string $region,
    ): self {
        return new self(
            eventBusName: $eventBusName,
            detailType: $detailType,
            accessKeyId: $accessKeyId,
            region: $region,
            setFields: ['eventBusName' => true, 'detailType' => true, 'accessKeyId' => true, 'region' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'eventBusName' => $this->eventBusName,
            'detailType' => $this->detailType,
            'accessKeyId' => $this->accessKeyId,
            'region' => $this->region];

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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'EventBridgeConfigOut'),
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
