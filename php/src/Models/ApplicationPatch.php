<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $metadata
     * @param int|null                   $throttleRate Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null $uid the Application's UID
     */
    private function __construct(
        public readonly ?array $metadata = null,
        public readonly ?string $name = null,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            metadata: null,
            name: null,
            throttleRate: null,
            uid: null,
            setFields: []
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            metadata: $metadata,
            name: $this->name,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withName(?string $name): self
    {
        $setFields = $this->setFields;
        $setFields['name'] = true;

        return new self(
            metadata: $this->metadata,
            name: $name,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            metadata: $this->metadata,
            name: $this->name,
            throttleRate: $throttleRate,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            metadata: $this->metadata,
            name: $this->name,
            throttleRate: $this->throttleRate,
            uid: $uid,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
        }
        if (null !== $this->name) {
            $data['name'] = $this->name;
        }
        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'ApplicationPatch'),
            name: \Svix\Utils::deserializeString($data, 'name', false, 'ApplicationPatch'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'ApplicationPatch'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ApplicationPatch')
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
