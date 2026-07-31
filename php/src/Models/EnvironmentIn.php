<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EnvironmentIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<EventTypeIn>|null $eventTypes
     * @param list<ConnectorIn>|null $connectors
     */
    private function __construct(
        public readonly ?array $eventTypes = null,
        public readonly ?array $settings = null,
        public readonly ?array $connectors = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EnvironmentIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            eventTypes: null,
            settings: null,
            connectors: null,
            setFields: []
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            eventTypes: $eventTypes,
            settings: $this->settings,
            connectors: $this->connectors,
            setFields: $setFields
        );
    }

    public function withSettings(?array $settings): self
    {
        $setFields = $this->setFields;
        $setFields['settings'] = true;

        return new self(
            eventTypes: $this->eventTypes,
            settings: $settings,
            connectors: $this->connectors,
            setFields: $setFields
        );
    }

    public function withConnectors(?array $connectors): self
    {
        $setFields = $this->setFields;
        $setFields['connectors'] = true;

        return new self(
            eventTypes: $this->eventTypes,
            settings: $this->settings,
            connectors: $connectors,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['eventTypes'])) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (isset($this->setFields['settings'])) {
            $data['settings'] = $this->settings;
        }
        if (isset($this->setFields['connectors'])) {
            $data['connectors'] = $this->connectors;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            eventTypes: \Svix\Utils::deserializeObjectArray($data, 'eventTypes', true, 'EnvironmentIn', [EventTypeIn::class, 'fromMixed']),
            settings: \Svix\Utils::getValFromJson($data, 'settings', false, 'EnvironmentIn'),
            connectors: \Svix\Utils::deserializeObjectArray($data, 'connectors', true, 'EnvironmentIn', [ConnectorIn::class, 'fromMixed'])
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
