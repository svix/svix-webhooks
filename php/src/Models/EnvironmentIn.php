<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EnvironmentIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<ConnectorIn>|null $connectors
     * @param list<EventTypeIn>|null $eventTypes
     */
    private function __construct(
        public readonly ?array $connectors = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $settings = null,
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
            connectors: null,
            eventTypes: null,
            settings: null,
            setFields: []
        );
    }

    public function withConnectors(?array $connectors): self
    {
        $setFields = $this->setFields;
        $setFields['connectors'] = true;

        return new self(
            connectors: $connectors,
            eventTypes: $this->eventTypes,
            settings: $this->settings,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            connectors: $this->connectors,
            eventTypes: $eventTypes,
            settings: $this->settings,
            setFields: $setFields
        );
    }

    public function withSettings(?array $settings): self
    {
        $setFields = $this->setFields;
        $setFields['settings'] = true;

        return new self(
            connectors: $this->connectors,
            eventTypes: $this->eventTypes,
            settings: $settings,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['connectors'])) {
            $data['connectors'] = $this->connectors;
        }
        if (isset($this->setFields['eventTypes'])) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (isset($this->setFields['settings'])) {
            $data['settings'] = $this->settings;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            connectors: \Svix\Utils::deserializeObjectArray($data, 'connectors', true, 'EnvironmentIn', [ConnectorIn::class, 'fromMixed']),
            eventTypes: \Svix\Utils::deserializeObjectArray($data, 'eventTypes', true, 'EnvironmentIn', [EventTypeIn::class, 'fromMixed']),
            settings: \Svix\Utils::getValFromJson($data, 'settings', false, 'EnvironmentIn')
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
