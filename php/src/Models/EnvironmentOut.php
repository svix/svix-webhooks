<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EnvironmentOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<EventTypeOut> $eventTypes
     * @param list<ConnectorOut> $transformationTemplates
     */
    private function __construct(
        public readonly \DateTimeImmutable $createdAt,
        public readonly array $eventTypes,
        public readonly array $settings,
        public readonly array $transformationTemplates,
        public readonly ?int $version = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EnvironmentOut with required fields.
     */
    public static function create(
        \DateTimeImmutable $createdAt,
        array $eventTypes,
        array $settings,
        array $transformationTemplates,
    ): self {
        return new self(
            createdAt: $createdAt,
            eventTypes: $eventTypes,
            settings: $settings,
            transformationTemplates: $transformationTemplates,
            version: null,
            setFields: ['createdAt' => true, 'eventTypes' => true, 'settings' => true, 'transformationTemplates' => true]
        );
    }

    public function withVersion(?int $version): self
    {
        $setFields = $this->setFields;
        $setFields['version'] = true;

        return new self(
            createdAt: $this->createdAt,
            eventTypes: $this->eventTypes,
            settings: $this->settings,
            transformationTemplates: $this->transformationTemplates,
            version: $version,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'createdAt' => $this->createdAt->format('c'),
            'eventTypes' => $this->eventTypes,
            'transformationTemplates' => $this->transformationTemplates];

        if (null !== $this->version) {
            $data['version'] = $this->version;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'EnvironmentOut'),
            eventTypes: \Svix\Utils::deserializeObjectArray($data, 'eventTypes', true, 'EnvironmentOut', [EventTypeOut::class, 'fromMixed']),
            settings: \Svix\Utils::getValFromJson($data, 'settings', true, 'EnvironmentOut'),
            transformationTemplates: \Svix\Utils::deserializeObjectArray($data, 'transformationTemplates', true, 'EnvironmentOut', [ConnectorOut::class, 'fromMixed']),
            version: \Svix\Utils::deserializeInt($data, 'version', false, 'EnvironmentOut')
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
