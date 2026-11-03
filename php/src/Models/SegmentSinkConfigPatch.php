<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SegmentSinkConfigPatch implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $host = null,
        public readonly ?string $writeKey = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SegmentSinkConfigPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            host: null,
            writeKey: null,
            setFields: []
        );
    }

    public function withHost(?string $host): self
    {
        $setFields = $this->setFields;
        $setFields['host'] = true;

        return new self(
            host: $host,
            writeKey: $this->writeKey,
            setFields: $setFields
        );
    }

    public function withWriteKey(?string $writeKey): self
    {
        $setFields = $this->setFields;
        $setFields['writeKey'] = true;

        return new self(
            host: $this->host,
            writeKey: $writeKey,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->host) {
            $data['host'] = $this->host;
        }
        if (null !== $this->writeKey) {
            $data['writeKey'] = $this->writeKey;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            host: \Svix\Utils::getValFromJson($data, 'host', false, 'SegmentSinkConfigPatch'),
            writeKey: \Svix\Utils::deserializeString($data, 'writeKey', false, 'SegmentSinkConfigPatch')
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
