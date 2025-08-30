<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeImportOpenApiOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly EventTypeImportOpenApiOutData $data,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeImportOpenApiOut with required fields.
     */
    public static function create(
        EventTypeImportOpenApiOutData $data,
    ): self {
        return new self(
            data: $data,
            setFields: ['data' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['data' => $this->data];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObject($data, 'data', true, 'EventTypeImportOpenApiOut', [EventTypeImportOpenApiOutData::class, 'fromMixed'])
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
