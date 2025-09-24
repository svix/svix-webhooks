<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when an ingest endpoint has been automatically disabled after continuous failures, or manually via an API call. */
class IngestEndpointDisabledEventData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $endpointId  the Endpoint's ID
     * @param string|null $endpointUid the Endpoint's UID
     * @param string      $sourceId    the Source's ID
     */
    private function __construct(
        public readonly string $endpointId,
        public readonly string $sourceId,
        public readonly ?string $endpointUid = null,
        public readonly ?\DateTimeImmutable $failSince = null,
        public readonly ?EndpointDisabledTrigger $trigger = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestEndpointDisabledEventData with required fields.
     */
    public static function create(
        string $endpointId,
        string $sourceId,
    ): self {
        return new self(
            endpointId: $endpointId,
            endpointUid: null,
            failSince: null,
            sourceId: $sourceId,
            trigger: null,
            setFields: ['endpointId' => true, 'sourceId' => true]
        );
    }

    public function withEndpointUid(?string $endpointUid): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUid'] = true;

        return new self(
            endpointId: $this->endpointId,
            endpointUid: $endpointUid,
            failSince: $this->failSince,
            sourceId: $this->sourceId,
            trigger: $this->trigger,
            setFields: $setFields
        );
    }

    public function withFailSince(?\DateTimeImmutable $failSince): self
    {
        $setFields = $this->setFields;
        $setFields['failSince'] = true;

        return new self(
            endpointId: $this->endpointId,
            endpointUid: $this->endpointUid,
            failSince: $failSince,
            sourceId: $this->sourceId,
            trigger: $this->trigger,
            setFields: $setFields
        );
    }

    public function withTrigger(?EndpointDisabledTrigger $trigger): self
    {
        $setFields = $this->setFields;
        $setFields['trigger'] = true;

        return new self(
            endpointId: $this->endpointId,
            endpointUid: $this->endpointUid,
            failSince: $this->failSince,
            sourceId: $this->sourceId,
            trigger: $trigger,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'endpointId' => $this->endpointId,
            'sourceId' => $this->sourceId];

        if (isset($this->setFields['endpointUid'])) {
            $data['endpointUid'] = $this->endpointUid;
        }
        if (isset($this->setFields['failSince'])) {
            $data['failSince'] = $this->failSince->format('c');
        }
        if (null !== $this->trigger) {
            $data['trigger'] = $this->trigger;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'IngestEndpointDisabledEventData'),
            endpointUid: \Svix\Utils::deserializeString($data, 'endpointUid', false, 'IngestEndpointDisabledEventData'),
            failSince: \Svix\Utils::deserializeDt($data, 'failSince', false, 'IngestEndpointDisabledEventData'),
            sourceId: \Svix\Utils::deserializeString($data, 'sourceId', true, 'IngestEndpointDisabledEventData'),
            trigger: \Svix\Utils::deserializeObject($data, 'trigger', false, 'IngestEndpointDisabledEventData', [EndpointDisabledTrigger::class, 'fromMixed'])
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
