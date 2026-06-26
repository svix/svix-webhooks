<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration for a Google Cloud BigQuery sink. */
class BigQueryConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $credentials google Cloud Credentials JSON Object as a string
     */
    private function __construct(
        public readonly string $credentials,
        public readonly string $datasetId,
        public readonly string $projectId,
        public readonly string $tableId,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BigQueryConfig with required fields.
     */
    public static function create(
        string $credentials,
        string $datasetId,
        string $projectId,
        string $tableId,
    ): self {
        return new self(
            credentials: $credentials,
            datasetId: $datasetId,
            projectId: $projectId,
            tableId: $tableId,
            setFields: ['credentials' => true, 'datasetId' => true, 'projectId' => true, 'tableId' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'credentials' => $this->credentials,
            'datasetId' => $this->datasetId,
            'projectId' => $this->projectId,
            'tableId' => $this->tableId];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            credentials: \Svix\Utils::deserializeString($data, 'credentials', true, 'BigQueryConfig'),
            datasetId: \Svix\Utils::deserializeString($data, 'datasetId', true, 'BigQueryConfig'),
            projectId: \Svix\Utils::deserializeString($data, 'projectId', true, 'BigQueryConfig'),
            tableId: \Svix\Utils::deserializeString($data, 'tableId', true, 'BigQueryConfig')
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
