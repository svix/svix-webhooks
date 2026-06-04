<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClickhouseConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string|null $database  The Clickhouse database to connect to
     * @param string      $password  Password to access Clickhouse
     * @param string      $tableName The Clickhouse table to write to
     * @param string      $url       The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
     * @param string      $username  Username to access Clickhouse
     */
    private function __construct(
        public readonly string $password,
        public readonly string $tableName,
        public readonly string $url,
        public readonly string $username,
        public readonly ?string $database = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ClickhouseConfig with required fields.
     */
    public static function create(
        string $password,
        string $tableName,
        string $url,
        string $username,
    ): self {
        return new self(
            database: null,
            password: $password,
            tableName: $tableName,
            url: $url,
            username: $username,
            setFields: ['password' => true, 'tableName' => true, 'url' => true, 'username' => true]
        );
    }

    public function withDatabase(?string $database): self
    {
        $setFields = $this->setFields;
        $setFields['database'] = true;

        return new self(
            database: $database,
            password: $this->password,
            tableName: $this->tableName,
            url: $this->url,
            username: $this->username,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'password' => $this->password,
            'tableName' => $this->tableName,
            'url' => $this->url,
            'username' => $this->username];

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
            database: \Svix\Utils::deserializeString($data, 'database', false, 'ClickhouseConfig'),
            password: \Svix\Utils::deserializeString($data, 'password', true, 'ClickhouseConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', true, 'ClickhouseConfig'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'ClickhouseConfig'),
            username: \Svix\Utils::deserializeString($data, 'username', true, 'ClickhouseConfig')
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
