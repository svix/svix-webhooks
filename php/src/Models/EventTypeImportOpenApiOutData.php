<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EventTypeImportOpenApiOutData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>                    $modified
     * @param list<EventTypeFromOpenApi>|null $toModify
     */
    private function __construct(
        public readonly array $modified,
        public readonly ?array $toModify = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EventTypeImportOpenApiOutData with required fields.
     */
    public static function create(
        array $modified,
    ): self {
        return new self(
            modified: $modified,
            toModify: null,
            setFields: ['modified' => true]
        );
    }

    public function withToModify(?array $toModify): self
    {
        $setFields = $this->setFields;
        $setFields['toModify'] = true;

        return new self(
            modified: $this->modified,
            toModify: $toModify,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'modified' => $this->modified];

        if (isset($this->setFields['toModify'])) {
            $data['to_modify'] = $this->toModify;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            modified: \Svix\Utils::getValFromJson($data, 'modified', true, 'EventTypeImportOpenApiOutData'),
            toModify: \Svix\Utils::deserializeObjectArray($data, 'to_modify', true, 'EventTypeImportOpenApiOutData', [EventTypeFromOpenApi::class, 'fromMixed'])
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
