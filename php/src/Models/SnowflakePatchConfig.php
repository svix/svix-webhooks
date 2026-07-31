<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SnowflakePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $dbName Database name.
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
        public readonly ?string $privateKey = null,
        public readonly ?string $accountIdentifier = null,
        public readonly ?string $userId = null,
        public readonly ?string $dbName = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SnowflakePatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            privateKey: null,
            accountIdentifier: null,
            userId: null,
            dbName: null,
            schemaName: null,
            tableName: null,
            setFields: []
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

    public function withAccountIdentifier(?string $accountIdentifier): self
    {
        $setFields = $this->setFields;
        $setFields['accountIdentifier'] = true;

        return new self(
            privateKey: $this->privateKey,
            accountIdentifier: $accountIdentifier,
            userId: $this->userId,
            dbName: $this->dbName,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withUserId(?string $userId): self
    {
        $setFields = $this->setFields;
        $setFields['userId'] = true;

        return new self(
            privateKey: $this->privateKey,
            accountIdentifier: $this->accountIdentifier,
            userId: $userId,
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
        ];

        if (null !== $this->privateKey) {
            $data['privateKey'] = $this->privateKey;
        }
        if (null !== $this->accountIdentifier) {
            $data['accountIdentifier'] = $this->accountIdentifier;
        }
        if (null !== $this->userId) {
            $data['userId'] = $this->userId;
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
            privateKey: \Svix\Utils::deserializeString($data, 'privateKey', false, 'SnowflakePatchConfig'),
            accountIdentifier: \Svix\Utils::deserializeString($data, 'accountIdentifier', false, 'SnowflakePatchConfig'),
            userId: \Svix\Utils::deserializeString($data, 'userId', false, 'SnowflakePatchConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'SnowflakePatchConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'SnowflakePatchConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'SnowflakePatchConfig')
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
