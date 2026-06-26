<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class RedshiftPatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $dbName Database name.
     *
     * Only required if not using transformations.
     * @param string|null $schemaName Schema name.
     *
     * Only used if not using transformations.
     * @param string|null $tableName Table name.
     *
     * Only required if not using transformations.
     */
    private function __construct(
        public readonly ?string $accessKeyId = null,
        public readonly ?string $dbName = null,
        public readonly ?string $region = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $tableName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RedshiftPatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            accessKeyId: null,
            dbName: null,
            region: null,
            schemaName: null,
            secretAccessKey: null,
            tableName: null,
            setFields: []
        );
    }

    public function withAccessKeyId(?string $accessKeyId): self
    {
        $setFields = $this->setFields;
        $setFields['accessKeyId'] = true;

        return new self(
            accessKeyId: $accessKeyId,
            dbName: $this->dbName,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withDbName(?string $dbName): self
    {
        $setFields = $this->setFields;
        $setFields['dbName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            dbName: $dbName,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withRegion(?string $region): self
    {
        $setFields = $this->setFields;
        $setFields['region'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            dbName: $this->dbName,
            region: $region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withSchemaName(?string $schemaName): self
    {
        $setFields = $this->setFields;
        $setFields['schemaName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            dbName: $this->dbName,
            region: $this->region,
            schemaName: $schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withSecretAccessKey(?string $secretAccessKey): self
    {
        $setFields = $this->setFields;
        $setFields['secretAccessKey'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            dbName: $this->dbName,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $secretAccessKey,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            dbName: $this->dbName,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $tableName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->accessKeyId) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (null !== $this->dbName) {
            $data['dbName'] = $this->dbName;
        }
        if (null !== $this->region) {
            $data['region'] = $this->region;
        }
        if (isset($this->setFields['schemaName'])) {
            $data['schemaName'] = $this->schemaName;
        }
        if (null !== $this->secretAccessKey) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (null !== $this->tableName) {
            $data['tableName'] = $this->tableName;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'RedshiftPatchConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'RedshiftPatchConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'RedshiftPatchConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'RedshiftPatchConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'RedshiftPatchConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'RedshiftPatchConfig')
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
