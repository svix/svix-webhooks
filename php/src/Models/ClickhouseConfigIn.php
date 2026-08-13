<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClickhouseConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $url      The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
     * @param string|null $username Username to access Clickhouse.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $password Password to access Clickhouse.
     *
     * Currently a required field, but marked as optional because we may add different authentication in the future.
     * @param string|null $database  the Clickhouse database to connect to
     * @param string      $tableName the Clickhouse table to write to
     */
    private function __construct(
        public readonly string $url,
        public readonly string $tableName,
        public readonly ?string $username = null,
        public readonly ?string $password = null,
        public readonly ?string $database = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ClickhouseConfigIn with required fields.
     */
    public static function create(
        string $url,
        string $tableName,
    ): self {
        return new self(
            url: $url,
            username: null,
            password: null,
            database: null,
            tableName: $tableName,
            setFields: ['url' => true, 'tableName' => true]
        );
    }

    public function withUsername(?string $username): self
    {
        $setFields = $this->setFields;
        $setFields['username'] = true;

        return new self(
            url: $this->url,
            username: $username,
            password: $this->password,
            database: $this->database,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withPassword(?string $password): self
    {
        $setFields = $this->setFields;
        $setFields['password'] = true;

        return new self(
            url: $this->url,
            username: $this->username,
            password: $password,
            database: $this->database,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function withDatabase(?string $database): self
    {
        $setFields = $this->setFields;
        $setFields['database'] = true;

        return new self(
            url: $this->url,
            username: $this->username,
            password: $this->password,
            database: $database,
            tableName: $this->tableName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url,
            'tableName' => $this->tableName];

        if (isset($this->setFields['username'])) {
            $data['username'] = $this->username;
        }
        if (isset($this->setFields['password'])) {
            $data['password'] = $this->password;
        }
        if (null !== $this->database) {
            $data['database'] = $this->database;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'ClickhouseConfigIn'),
            username: \Svix\Utils::deserializeString($data, 'username', false, 'ClickhouseConfigIn'),
            password: \Svix\Utils::deserializeString($data, 'password', false, 'ClickhouseConfigIn'),
            database: \Svix\Utils::deserializeString($data, 'database', false, 'ClickhouseConfigIn'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', true, 'ClickhouseConfigIn')
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
