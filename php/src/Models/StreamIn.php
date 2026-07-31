<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                     $name     the stream's name
     * @param string|null                $uid      an optional unique identifier for the stream
     * @param array<string, string>|null $metadata
     */
    private function __construct(
        public readonly string $name,
        public readonly ?string $uid = null,
        public readonly ?array $metadata = null,
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
            name: $name,
            uid: null,
            metadata: null,
            setFields: ['name' => true]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            name: $this->name,
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
            uid: $this->uid,
            metadata: $metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'name' => $this->name];

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
            name: \Svix\Utils::deserializeString($data, 'name', true, 'StreamIn'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamIn'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'StreamIn')
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
