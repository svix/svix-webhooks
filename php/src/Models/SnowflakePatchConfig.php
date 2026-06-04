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
        public readonly ?string $accountIdentifier = null,
        public readonly ?string $dbName = null,
        public readonly ?string $privateKey = null,
        public readonly ?string $schemaName = null,
        public readonly ?string $tableName = null,
        public readonly ?string $userId = null,
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
            accountIdentifier: null,
            dbName: null,
            privateKey: null,
            schemaName: null,
            tableName: null,
            userId: null,
            setFields: []
        );
    }

    public function withAccountIdentifier(?string $accountIdentifier): self
    {
        $setFields = $this->setFields;
        $setFields['accountIdentifier'] = true;

        return new self(
            accountIdentifier: $accountIdentifier,
            dbName: $this->dbName,
            privateKey: $this->privateKey,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            userId: $this->userId,
            setFields: $setFields
        );
    }

    public function withDbName(?string $dbName): self
    {
        $setFields = $this->setFields;
        $setFields['dbName'] = true;

        return new self(
            accountIdentifier: $this->accountIdentifier,
            dbName: $dbName,
            privateKey: $this->privateKey,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            userId: $this->userId,
            setFields: $setFields
        );
    }

    public function withPrivateKey(?string $privateKey): self
    {
        $setFields = $this->setFields;
        $setFields['privateKey'] = true;

        return new self(
            accountIdentifier: $this->accountIdentifier,
            dbName: $this->dbName,
            privateKey: $privateKey,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            userId: $this->userId,
            setFields: $setFields
        );
    }

    public function withSchemaName(?string $schemaName): self
    {
        $setFields = $this->setFields;
        $setFields['schemaName'] = true;

        return new self(
            accountIdentifier: $this->accountIdentifier,
            dbName: $this->dbName,
            privateKey: $this->privateKey,
            schemaName: $schemaName,
            tableName: $this->tableName,
            userId: $this->userId,
            setFields: $setFields
        );
    }

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            accountIdentifier: $this->accountIdentifier,
            dbName: $this->dbName,
            privateKey: $this->privateKey,
            schemaName: $this->schemaName,
            tableName: $tableName,
            userId: $this->userId,
            setFields: $setFields
        );
    }

    public function withUserId(?string $userId): self
    {
        $setFields = $this->setFields;
        $setFields['userId'] = true;

        return new self(
            accountIdentifier: $this->accountIdentifier,
            dbName: $this->dbName,
            privateKey: $this->privateKey,
            schemaName: $this->schemaName,
            tableName: $this->tableName,
            userId: $userId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->accountIdentifier) {
            $data['accountIdentifier'] = $this->accountIdentifier;
        }
        if (null !== $this->dbName) {
            $data['dbName'] = $this->dbName;
        }
        if (null !== $this->privateKey) {
            $data['privateKey'] = $this->privateKey;
        }
        if (null !== $this->schemaName) {
            $data['schemaName'] = $this->schemaName;
        }
        if (null !== $this->tableName) {
            $data['tableName'] = $this->tableName;
        }
        if (null !== $this->userId) {
            $data['userId'] = $this->userId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            accountIdentifier: \Svix\Utils::deserializeString($data, 'accountIdentifier', false, 'SnowflakePatchConfig'),
            dbName: \Svix\Utils::deserializeString($data, 'dbName', false, 'SnowflakePatchConfig'),
            privateKey: \Svix\Utils::deserializeString($data, 'privateKey', false, 'SnowflakePatchConfig'),
            schemaName: \Svix\Utils::deserializeString($data, 'schemaName', false, 'SnowflakePatchConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'SnowflakePatchConfig'),
            userId: \Svix\Utils::deserializeString($data, 'userId', false, 'SnowflakePatchConfig')
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
