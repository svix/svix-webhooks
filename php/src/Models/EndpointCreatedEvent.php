<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when an endpoint is created. */
class EndpointCreatedEvent implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly EndpointCreatedEventData $data,
        public readonly string $type,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointCreatedEvent with required fields.
     */
    public static function create(
        EndpointCreatedEventData $data,
        string $type,
    ): self {
        return new self(
            data: $data,
            type: $type,
            setFields: ['data' => true, 'type' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'type' => $this->type];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObject($data, 'data', true, 'EndpointCreatedEvent', [EndpointCreatedEventData::class, 'fromMixed']),
            type: \Svix\Utils::getValFromJson($data, 'type', true, 'EndpointCreatedEvent')
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
