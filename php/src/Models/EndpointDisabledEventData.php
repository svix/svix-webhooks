<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call. */
class EndpointDisabledEventData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $appId       the Application's ID
     * @param string|null $appUid      the Application's UID
     * @param string      $endpointId  the Endpoint's ID
     * @param string|null $endpointUid the Endpoint's UID
     */
    private function __construct(
        public readonly string $appId,
        public readonly string $endpointId,
        public readonly ?string $appUid = null,
        public readonly ?string $endpointUid = null,
        public readonly ?\DateTimeImmutable $failSince = null,
        public readonly ?EndpointDisabledTrigger $trigger = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointDisabledEventData with required fields.
     */
    public static function create(
        string $appId,
        string $endpointId,
    ): self {
        return new self(
            appId: $appId,
            appUid: null,
            endpointId: $endpointId,
            endpointUid: null,
            failSince: null,
            trigger: null,
            setFields: ['appId' => true, 'endpointId' => true]
        );
    }

    public function withAppUid(?string $appUid): self
    {
        $setFields = $this->setFields;
        $setFields['appUid'] = true;

        return new self(
            appId: $this->appId,
            appUid: $appUid,
            endpointId: $this->endpointId,
            endpointUid: $this->endpointUid,
            failSince: $this->failSince,
            trigger: $this->trigger,
            setFields: $setFields
        );
    }

    public function withEndpointUid(?string $endpointUid): self
    {
        $setFields = $this->setFields;
        $setFields['endpointUid'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            endpointId: $this->endpointId,
            endpointUid: $endpointUid,
            failSince: $this->failSince,
            trigger: $this->trigger,
            setFields: $setFields
        );
    }

    public function withFailSince(?\DateTimeImmutable $failSince): self
    {
        $setFields = $this->setFields;
        $setFields['failSince'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            endpointId: $this->endpointId,
            endpointUid: $this->endpointUid,
            failSince: $failSince,
            trigger: $this->trigger,
            setFields: $setFields
        );
    }

    public function withTrigger(?EndpointDisabledTrigger $trigger): self
    {
        $setFields = $this->setFields;
        $setFields['trigger'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            endpointId: $this->endpointId,
            endpointUid: $this->endpointUid,
            failSince: $this->failSince,
            trigger: $trigger,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['appId' => $this->appId,
            'endpointId' => $this->endpointId];

        if (isset($this->setFields['appUid'])) {
            $data['appUid'] = $this->appUid;
        }
        if (isset($this->setFields['endpointUid'])) {
            $data['endpointUid'] = $this->endpointUid;
        }
        if (isset($this->setFields['failSince'])) {
            $data['failSince'] = $this->failSince->format('c');
        }
        if (null !== $this->trigger) {
            $data['trigger'] = $this->trigger;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            appId: \Svix\Utils::deserializeString($data, 'appId', true, 'EndpointDisabledEventData'),
            appUid: \Svix\Utils::deserializeString($data, 'appUid', false, 'EndpointDisabledEventData'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'EndpointDisabledEventData'),
            endpointUid: \Svix\Utils::deserializeString($data, 'endpointUid', false, 'EndpointDisabledEventData'),
            failSince: \Svix\Utils::deserializeDt($data, 'failSince', false, 'EndpointDisabledEventData'),
            trigger: \Svix\Utils::deserializeObject($data, 'trigger', false, 'EndpointDisabledEventData', [EndpointDisabledTrigger::class, 'fromMixed'])
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
