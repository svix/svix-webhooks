<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class CreateStreamEventsOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of CreateStreamEventsOut with required fields.
     */
    public static function create(
    ): self {
        return new self(
            setFields: []
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
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
