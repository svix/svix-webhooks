<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BigQueryConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $projectId,
        public readonly string $datasetId,
        public readonly string $tableId,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BigQueryConfigOut with required fields.
     */
    public static function create(
        string $projectId,
        string $datasetId,
        string $tableId,
    ): self {
        return new self(
            projectId: $projectId,
            datasetId: $datasetId,
            tableId: $tableId,
            setFields: ['projectId' => true, 'datasetId' => true, 'tableId' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'projectId' => $this->projectId,
            'datasetId' => $this->datasetId,
            'tableId' => $this->tableId];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            projectId: \Svix\Utils::deserializeString($data, 'projectId', true, 'BigQueryConfigOut'),
            datasetId: \Svix\Utils::deserializeString($data, 'datasetId', true, 'BigQueryConfigOut'),
            tableId: \Svix\Utils::deserializeString($data, 'tableId', true, 'BigQueryConfigOut')
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
