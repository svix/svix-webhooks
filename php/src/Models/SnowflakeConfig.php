<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration parameters for defining a Snowflake sink. */
class SnowflakeConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $privateKey PEM-encoded private key used for signing token-based requests to the Snowflake API.
     *
     * Beginning/end delimiters are not required.
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
        public readonly string $privateKey,
        public readonly string $accountIdentifier,
        public readonly string $userId,
        public readonly ?string $dbName = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnowflakeConfig with required fields.
     */
    public static function create(
        string $privateKey,
        string $accountIdentifier,
        string $userId,
    ): self {
        return new self(
            privateKey: $privateKey,
            accountIdentifier: $accountIdentifier,
            userId: $userId,
            dbName: null,
            schemaName: null,
            tableName: null,
            setFields: ['privateKey' => true, 'accountIdentifier' => true, 'userId' => true]
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
            'privateKey' => $this->privateKey,
            'accountIdentifier' => $this->accountIdentifier,
            'userId' => $this->userId];

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
            privateKey: \Svix\Utils::deserializeString($data, 'privateKey', true, 'SnowflakeConfig'),
            accountIdentifier: \Svix\Utils::deserializeString($data, 'accountIdentifier', true, 'SnowflakeConfig'),
            userId: \Svix\Utils::deserializeString($data, 'userId', true, 'SnowflakeConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'SnowflakeConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'SnowflakeConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'SnowflakeConfig')
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
