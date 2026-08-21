<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BulkExpungeContentsIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<string>|null $ids Message ID or UID to delete
     */
    private function __construct(
        public readonly ?array $ids = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BulkExpungeContentsIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            ids: null,
            setFields: []
        );
    }

    public function withIds(?array $ids): self
    {
        $setFields = $this->setFields;
        $setFields['ids'] = true;

        return new self(
            ids: $ids,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->ids) {
            $data['ids'] = $this->ids;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            ids: \Svix\Utils::getValFromJson($data, 'ids', false, 'BulkExpungeContentsIn')
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
