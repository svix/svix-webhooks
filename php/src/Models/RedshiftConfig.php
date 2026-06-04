<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * Configuration parameters for defining a Redshift sink.
 *
 * For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
 */
class RedshiftConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $clusterIdentifier required for provisioned clusters
     * @param string|null $dbName            Database name.
     *
     * Only required if not using transformations.
     * @param string|null $dbUser     required for provisioned clusters
     * @param string|null $schemaName Schema name.
     *
     * Only used if not using transformations.
     * @param string|null $tableName Table name.
     *
     * Only required if not using transformations.
     * @param string|null $workgroupName required for Redshift Serverless
     */
    private function __construct(
        public readonly string $accessKeyId,
        public readonly string $region,
        public readonly string $secretAccessKey,
        public readonly ?string $clusterIdentifier = null,
        public readonly ?string $dbName = null,
        public readonly ?string $dbUser = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        public readonly ?string $workgroupName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RedshiftConfig with required fields.
     */
    public static function create(
        string $accessKeyId,
        string $region,
        string $secretAccessKey,
    ): self {
        return new self(
            accessKeyId: $accessKeyId,
            clusterIdentifier: null,
            dbName: null,
            dbUser: null,
            region: $region,
            schemaName: null,
            secretAccessKey: $secretAccessKey,
            tableName: null,
            workgroupName: null,
            setFields: ['accessKeyId' => true, 'region' => true, 'secretAccessKey' => true]
        );
    }

    public function withClusterIdentifier(?string $clusterIdentifier): self
    {
        $setFields = $this->setFields;
        $setFields['clusterIdentifier'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $clusterIdentifier,
            dbName: $this->dbName,
            dbUser: $this->dbUser,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            workgroupName: $this->workgroupName,
            setFields: $setFields
        );
    }

    public function withDbName(?string $dbName): self
    {
        $setFields = $this->setFields;
        $setFields['dbName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $this->clusterIdentifier,
            dbName: $dbName,
            dbUser: $this->dbUser,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            workgroupName: $this->workgroupName,
            setFields: $setFields
        );
    }

    public function withDbUser(?string $dbUser): self
    {
        $setFields = $this->setFields;
        $setFields['dbUser'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $this->clusterIdentifier,
            dbName: $this->dbName,
            dbUser: $dbUser,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            workgroupName: $this->workgroupName,
            setFields: $setFields
        );
    }

    public function withSchemaName(?string $schemaName): self
    {
        $setFields = $this->setFields;
        $setFields['schemaName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $this->clusterIdentifier,
            dbName: $this->dbName,
            dbUser: $this->dbUser,
            region: $this->region,
            schemaName: $schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            workgroupName: $this->workgroupName,
            setFields: $setFields
        );
    }

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $this->clusterIdentifier,
            dbName: $this->dbName,
            dbUser: $this->dbUser,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $tableName,
            workgroupName: $this->workgroupName,
            setFields: $setFields
        );
    }

    public function withWorkgroupName(?string $workgroupName): self
    {
        $setFields = $this->setFields;
        $setFields['workgroupName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            clusterIdentifier: $this->clusterIdentifier,
            dbName: $this->dbName,
            dbUser: $this->dbUser,
            region: $this->region,
            schemaName: $this->schemaName,
            secretAccessKey: $this->secretAccessKey,
            tableName: $this->tableName,
            workgroupName: $workgroupName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKeyId' => $this->accessKeyId,
            'region' => $this->region,
            'secretAccessKey' => $this->secretAccessKey];

        if (isset($this->setFields['clusterIdentifier'])) {
            $data['clusterIdentifier'] = $this->clusterIdentifier;
        }
        if (null !== $this->dbName) {
            $data['dbName'] = $this->dbName;
        }
        if (isset($this->setFields['dbUser'])) {
            $data['dbUser'] = $this->dbUser;
        }
        if (isset($this->setFields['schemaName'])) {
            $data['schemaName'] = $this->schemaName;
        }
        if (null !== $this->tableName) {
            $data['tableName'] = $this->tableName;
        }
        if (isset($this->setFields['workgroupName'])) {
            $data['workgroupName'] = $this->workgroupName;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'RedshiftConfig'),
            clusterIdentifier: \Svix\Utils::deserializeString($data, 'clusterIdentifier', false, 'RedshiftConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'RedshiftConfig'),
            dbUser: \Svix\Utils::deserializeString($data, 'dbUser', false, 'RedshiftConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'RedshiftConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'RedshiftConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'RedshiftConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'RedshiftConfig'),
            workgroupName: \Svix\Utils::deserializeString($data, 'workgroupName', false, 'RedshiftConfig')
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
