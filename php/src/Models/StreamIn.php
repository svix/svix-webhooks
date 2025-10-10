<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $metadata
     * @param string                     $name     the stream's name
     * @param string|null                $uid      an optional unique identifier for the stream
     */
    private function __construct(
        public readonly string $name,
        public readonly ?array $metadata = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamIn with required fields.
     */
    public static function create(
        string $name,
    ): self {
        return new self(
            metadata: null,
            name: $name,
            uid: null,
            setFields: ['name' => true]
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            metadata: $metadata,
            name: $this->name,
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
            uid: $uid,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name];

        if (null !== $this->metadata) {
            $data['metadata'] = $this->metadata;
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
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'StreamIn'),
            name: \Svix\Utils::deserializeString($data, 'name', true, 'StreamIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamIn')
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
