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
        public readonly string $projectId,
        public readonly string $datasetId,
        public readonly string $tableId,
        public readonly string $credentials,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BigQueryConfig with required fields.
     */
    public static function create(
        string $projectId,
        string $datasetId,
        string $tableId,
        string $credentials,
    ): self {
        return new self(
            projectId: $projectId,
            datasetId: $datasetId,
            tableId: $tableId,
            credentials: $credentials,
            setFields: ['projectId' => true, 'datasetId' => true, 'tableId' => true, 'credentials' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'projectId' => $this->projectId,
            'datasetId' => $this->datasetId,
            'tableId' => $this->tableId,
            'credentials' => $this->credentials];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            projectId: \Svix\Utils::deserializeString($data, 'projectId', true, 'BigQueryConfig'),
            datasetId: \Svix\Utils::deserializeString($data, 'datasetId', true, 'BigQueryConfig'),
            tableId: \Svix\Utils::deserializeString($data, 'tableId', true, 'BigQueryConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', true, 'BigQueryConfig')
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
