<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class BigQueryPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $projectId = null,
        public readonly ?string $datasetId = null,
        public readonly ?string $tableId = null,
        public readonly ?string $credentials = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of BigQueryPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            projectId: null,
            datasetId: null,
            tableId: null,
            credentials: null,
            setFields: []
        );
    }

    public function withProjectId(?string $projectId): self
    {
        $setFields = $this->setFields;
        $setFields['projectId'] = true;

        return new self(
            projectId: $projectId,
            datasetId: $this->datasetId,
            tableId: $this->tableId,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withDatasetId(?string $datasetId): self
    {
        $setFields = $this->setFields;
        $setFields['datasetId'] = true;

        return new self(
            projectId: $this->projectId,
            datasetId: $datasetId,
            tableId: $this->tableId,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withTableId(?string $tableId): self
    {
        $setFields = $this->setFields;
        $setFields['tableId'] = true;

        return new self(
            projectId: $this->projectId,
            datasetId: $this->datasetId,
            tableId: $tableId,
            credentials: $this->credentials,
            setFields: $setFields
        );
    }

    public function withCredentials(?string $credentials): self
    {
        $setFields = $this->setFields;
        $setFields['credentials'] = true;

        return new self(
            projectId: $this->projectId,
            datasetId: $this->datasetId,
            tableId: $this->tableId,
            credentials: $credentials,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->projectId) {
            $data['projectId'] = $this->projectId;
        }
        if (null !== $this->datasetId) {
            $data['datasetId'] = $this->datasetId;
        }
        if (null !== $this->tableId) {
            $data['tableId'] = $this->tableId;
        }
        if (null !== $this->credentials) {
            $data['credentials'] = $this->credentials;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            projectId: \Svix\Utils::deserializeString($data, 'projectId', false, 'BigQueryPatchConfig'),
            datasetId: \Svix\Utils::deserializeString($data, 'datasetId', false, 'BigQueryPatchConfig'),
            tableId: \Svix\Utils::deserializeString($data, 'tableId', false, 'BigQueryPatchConfig'),
            credentials: \Svix\Utils::deserializeString($data, 'credentials', false, 'BigQueryPatchConfig')
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
