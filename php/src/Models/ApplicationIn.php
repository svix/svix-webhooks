<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ApplicationIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string   $name         application name for human consumption
     * @param int|null $throttleRate Maximum messages per second to send to this application.
     *
     * Outgoing messages will be throttled to this rate.
     * @param string|null                $uid      optional unique identifier for the application
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly string $name,
        public readonly ?int $throttleRate = null,
        public readonly ?string $uid = null,
        public readonly ?array $metadata = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ApplicationIn with required fields.
     */
    public static function create(
        string $name,
    ): self {
        return new self(
            name: $name,
            throttleRate: null,
            uid: null,
            metadata: null,
            setFields: ['name' => true]
        );
    }

    public function withThrottleRate(?int $throttleRate): self
    {
        $setFields = $this->setFields;
        $setFields['throttleRate'] = true;

        return new self(
            name: $this->name,
            throttleRate: $throttleRate,
            uid: $this->uid,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            name: $this->name,
            throttleRate: $this->throttleRate,
            uid: $uid,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            name: $this->name,
            throttleRate: $this->throttleRate,
            uid: $this->uid,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name];

        if (isset($this->setFields['throttleRate'])) {
            $data['throttleRate'] = $this->throttleRate;
        }
        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
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
            name: \Svix\Utils::deserializeString($data, 'name', true, 'ApplicationIn'),
            throttleRate: \Svix\Utils::deserializeInt($data, 'throttleRate', false, 'ApplicationIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'ApplicationIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'ApplicationIn')
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
