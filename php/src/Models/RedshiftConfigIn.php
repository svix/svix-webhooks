<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * Configuration parameters for defining a Redshift sink.
 *
 * For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
 */
class RedshiftConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $accessKeyId Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $secretAccessKey Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $region The region of the Redshift DB.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields in the future.
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
        public readonly ?string $accessKeyId = null,
        public readonly ?string $secretAccessKey = null,
        public readonly ?string $region = null,
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
     * Create an instance of RedshiftConfigIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            accessKeyId: null,
            secretAccessKey: null,
            region: null,
            clusterIdentifier: null,
            dbUser: null,
            workgroupName: null,
            dbName: null,
            schemaName: null,
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
            secretAccessKey: $this->secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
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
            secretAccessKey: $secretAccessKey,
            region: $this->region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
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
            secretAccessKey: $this->secretAccessKey,
            region: $region,
            clusterIdentifier: $this->clusterIdentifier,
            dbUser: $this->dbUser,
            workgroupName: $this->workgroupName,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            setFields: $setFields
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
        ];

        if (isset($this->setFields['accessKeyId'])) {
            $data['accessKeyId'] = $this->accessKeyId;
        }
        if (isset($this->setFields['secretAccessKey'])) {
            $data['secretAccessKey'] = $this->secretAccessKey;
        }
        if (isset($this->setFields['region'])) {
            $data['region'] = $this->region;
        }
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
            accessKeyId: \Svix\Utils::deserializeString($data, 'accessKeyId', false, 'RedshiftConfigIn'),
            secretAccessKey: \Svix\Utils::deserializeString($data, 'secretAccessKey', false, 'RedshiftConfigIn'),
            region: \Svix\Utils::deserializeString($data, 'region', false, 'RedshiftConfigIn'),
            clusterIdentifier: \Svix\Utils::deserializeString($data, 'clusterIdentifier', false, 'RedshiftConfigIn'),
            dbUser: \Svix\Utils::deserializeString($data, 'dbUser', false, 'RedshiftConfigIn'),
            workgroupName: \Svix\Utils::deserializeString($data, 'workgroupName', false, 'RedshiftConfigIn'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'RedshiftConfigIn'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'RedshiftConfigIn'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'RedshiftConfigIn')
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
