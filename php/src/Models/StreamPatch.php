<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamPatch implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null                $description the Stream's description
     * @param array<string, string>|null $metadata
     * @param string|null                $uid         an optional unique identifier for the stream
     */
    private function __construct(
        public readonly ?string $description = null,
        public readonly ?array $metadata = null,
        public readonly ?string $uid = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            description: null,
            metadata: null,
            uid: null,
            setFields: []
        );
    }

    public function withDescription(?string $description): self
    {
        $setFields = $this->setFields;
        $setFields['description'] = true;

        return new self(
            description: $description,
            metadata: $this->metadata,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withMetadata(?array $metadata): self
    {
        $setFields = $this->setFields;
        $setFields['metadata'] = true;

        return new self(
            description: $this->description,
            metadata: $metadata,
            uid: $this->uid,
            setFields: $setFields
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            description: $this->description,
            metadata: $this->metadata,
            uid: $uid,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->description) {
            $data['description'] = $this->description;
        }
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
            description: \Svix\Utils::deserializeString($data, 'description', false, 'StreamPatch'),
            metadata: \Svix\Utils::getValFromJson($data, 'metadata', false, 'StreamPatch'),
            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamPatch')
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
