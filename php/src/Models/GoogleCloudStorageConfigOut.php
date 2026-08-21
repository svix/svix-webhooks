<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class GoogleCloudStorageConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $bucket,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of GoogleCloudStorageConfigOut with required fields.
     */
    public static function create(
        string $bucket,
    ): self {
        return new self(
            bucket: $bucket,
            setFields: ['bucket' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'bucket' => $this->bucket];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            bucket: \Svix\Utils::deserializeString($data, 'bucket', true, 'GoogleCloudStorageConfigOut')
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
