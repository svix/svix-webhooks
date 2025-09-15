<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when an endpoint is created, updated, or deleted */
class EndpointDeletedEventData implements \JsonSerializable
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
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointDeletedEventData with required fields.
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
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'appId' => $this->appId,
            'endpointId' => $this->endpointId];

        if (isset($this->setFields['appUid'])) {
            $data['appUid'] = $this->appUid;
        }
        if (isset($this->setFields['endpointUid'])) {
            $data['endpointUid'] = $this->endpointUid;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            appId: \Svix\Utils::deserializeString($data, 'appId', true, 'EndpointDeletedEventData'),
            appUid: \Svix\Utils::deserializeString($data, 'appUid', false, 'EndpointDeletedEventData'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'EndpointDeletedEventData'),
            endpointUid: \Svix\Utils::deserializeString($data, 'endpointUid', false, 'EndpointDeletedEventData')
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
