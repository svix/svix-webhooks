<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration parameters for defining a Snowflake sink. */
class SnowflakeConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $privateKey PEM-encoded private key used for signing token-based requests to the Snowflake API.
     *
     * Beginning/end delimiters are not required.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string      $accountIdentifier snowflake account identifier, which includes both the organization and account IDs separated by a hyphen
     * @param string      $userId            the Snowflake user id
     * @param string|null $dbName            Database name.
     *
     * Only required if not using transformations.
     * @param string|null $schemaName Schema name.
     *
     * Only required if not using transformations.
     * @param string|null $tableName Table name.
     *
     * Only required if not using transformations.
     */
    private function __construct(
        public readonly string $accountIdentifier,
        public readonly string $userId,
        public readonly ?string $privateKey = null,
        public readonly ?string $dbName = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnowflakeConfigIn with required fields.
     */
    public static function create(
        string $accountIdentifier,
        string $userId,
    ): self {
        return new self(
            privateKey: null,
            accountIdentifier: $accountIdentifier,
            userId: $userId,
            dbName: null,
            schemaName: null,
            tableName: null,
            setFields: ['accountIdentifier' => true, 'userId' => true]
        );
    }

    public function withPrivateKey(?string $privateKey): self
    {
        $setFields = $this->setFields;
        $setFields['privateKey'] = true;

        return new self(
            privateKey: $privateKey,
            accountIdentifier: $this->accountIdentifier,
            userId: $this->userId,
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
            privateKey: $this->privateKey,
            accountIdentifier: $this->accountIdentifier,
            userId: $this->userId,
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
            privateKey: $this->privateKey,
            accountIdentifier: $this->accountIdentifier,
            userId: $this->userId,
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
            privateKey: $this->privateKey,
            accountIdentifier: $this->accountIdentifier,
            userId: $this->userId,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $tableName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'accountIdentifier' => $this->accountIdentifier,
            'userId' => $this->userId];

        if (isset($this->setFields['privateKey'])) {
            $data['privateKey'] = $this->privateKey;
        }
        if (null !== $this->dbName) {
            $data['dbName'] = $this->dbName;
        }
        if (null !== $this->schemaName) {
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
            privateKey: \Svix\Utils::deserializeString($data, 'privateKey', false, 'SnowflakeConfigIn'),
            accountIdentifier: \Svix\Utils::deserializeString($data, 'accountIdentifier', true, 'SnowflakeConfigIn'),
            userId: \Svix\Utils::deserializeString($data, 'userId', true, 'SnowflakeConfigIn'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'SnowflakeConfigIn'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'SnowflakeConfigIn'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'SnowflakeConfigIn')
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
