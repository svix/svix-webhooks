<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class OperationalWebhookEndpointUpsertIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int|null $throttleRate Maximum messages per second to send to this endpoint.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null                $uid           optional unique identifier for the endpoint
     * @param list<string>|null          $eventTypesIds
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly string $url,
        public readonly ?string $description = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?bool $disabled = null,
        public readonly ?array $eventTypesIds = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of OperationalWebhookEndpointUpsertIn with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            description: null,
            throttleRate: null,
            uid: null,
            url: $url,
            disabled: null,
            eventTypesIds: null,
            metadata: null,
            setFields: ['url' => true]
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypesIds: $this->eventTypesIds,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            description: $this->description,
            throttleRate: $throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypesIds: $this->eventTypesIds,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypesIds: $this->eventTypesIds,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withDisabled(?bool $disabled): self
    {
        $setFields = $this->setFields;
        $setFields['disabled'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $disabled,
            eventTypesIds: $this->eventTypesIds,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withEventTypesIds(?array $eventTypesIds): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypesIds'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypesIds: $eventTypesIds,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            description: $this->description,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            url: $this->url,
            disabled: $this->disabled,
            eventTypesIds: $this->eventTypesIds,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (null !== $this->disabled) {
            $data['disabled'] = $this->disabled;
        }
        if (isset($this->setFields['eventTypesIds'])) {
            $data['eventTypesIds'] = $this->eventTypesIds;
        }
        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            description: \Svix\Utils::deserializeString($data, 'description', false, 'OperationalWebhookEndpointUpsertIn'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'OperationalWebhookEndpointUpsertIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'OperationalWebhookEndpointUpsertIn'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'OperationalWebhookEndpointUpsertIn'),
            disabled: \Svix\Utils::deserializeBool($data, 'disabled', false, 'OperationalWebhookEndpointUpsertIn'),
            eventTypesIds: \Svix\Utils::getValFromJson($data, 'eventTypesIds', false, 'OperationalWebhookEndpointUpsertIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'OperationalWebhookEndpointUpsertIn')
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
