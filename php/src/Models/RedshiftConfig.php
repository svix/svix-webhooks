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
     * @param string|null $dbUser            required for provisioned clusters
     * @param string|null $workgroupName     required for Redshift Serverless
     * @param string|null $dbName            Database name.
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
        public readonly string $accessKeyId,
        public readonly string $secretAccessKey,
        public readonly string $region,
        public readonly ?string $clusterIdentifier = null,
        public readonly ?string $dbUser = null,
        public readonly ?string $workgroupName = null,
        public readonly ?string $dbName = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of RedshiftConfig with required fields.
     */
    public static function create(
        string $accessKeyId,
        string $secretAccessKey,
        string $region,
    ): self {
        return new self(
            accessKeyId: $accessKeyId,
            secretAccessKey: $secretAccessKey,
            region: $region,
            clusterIdentifier: null,
            dbUser: null,
            workgroupName: null,
            dbName: null,
            schemaName: null,
            tableName: null,
            setFields: ['accessKeyId' => true, 'secretAccessKey' => true, 'region' => true]
        );
    }

    public function withClusterIdentifier(?string $clusterIdentifier): self
    {
        $setFields = $this->setFields;
        $setFields['clusterIdentifier'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withDbUser(?string $dbUser): self
    {
        $setFields = $this->setFields;
        $setFields['dbUser'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withWorkgroupName(?string $workgroupName): self
    {
        $setFields = $this->setFields;
        $setFields['workgroupName'] = true;

        return new self(
            accessKeyId: $this->accessKeyId,
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
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
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $dbName,
            schemaName: $this->schemaName,
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
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $schemaName,
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
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $tableName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accessKeyId' => $this->accessKeyId,
            'secretAccessKey' => $this->secretAccessKey,
            'region' => $this->region];

        if (isset($this->setFields['clusterIdentifier'])) {
            $data['clusterIdentifier'] = $this->clusterIdentifier;
        }
        if (isset($this->setFields['dbUser'])) {
            $data['dbUser'] = $this->dbUser;
        }
        if (isset($this->setFields['workgroupName'])) {
            $data['workgroupName'] = $this->workgroupName;
        }
        if (null !== $this->dbName) {
            $data['dbName'] = $this->dbName;
        }
        if (isset($this->setFields['schemaName'])) {
            $data['schemaName'] = $this->schemaName;
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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', true, 'RedshiftConfig'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', true, 'RedshiftConfig'),
            region: \Svix\Utils::deserializeString($data, 'region', true, 'RedshiftConfig'),
            clusterIdentifier: \Svix\Utils::deserializeString($data, 'clusterIdentifier', false, 'RedshiftConfig'),
            dbUser: \Svix\Utils::deserializeString($data, 'dbUser', false, 'RedshiftConfig'),
            workgroupName: \Svix\Utils::deserializeString($data, 'workgroupName', false, 'RedshiftConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'RedshiftConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'RedshiftConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'RedshiftConfig')
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
