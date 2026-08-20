<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BulkExpungeContentsOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, BulkExpungeStatus> $results Results of expunging (by ID)
     */
    private function __construct(
        public readonly array $results,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BulkExpungeContentsOut with required fields.
     */
    public static function create(
        array $results,
    ): self {
        return new self(
            results: $results,
            setFields: ['results' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'results' => $this->results];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            results: \Svix\Utils::getValFromJson($data, 'results', true, 'BulkExpungeContentsOut')
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
