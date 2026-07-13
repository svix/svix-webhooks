<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClickhouseConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $url       The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
     * @param string      $username  Username to access Clickhouse
     * @param string      $password  Password to access Clickhouse
     * @param string|null $database  The Clickhouse database to connect to
     * @param string      $tableName The Clickhouse table to write to
     */
    private function __construct(
        public readonly string $url,
        public readonly string $username,
        public readonly string $password,
        public readonly string $tableName,
        public readonly ?string $database = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ClickhouseConfig with required fields.
     */
    public static function create(
        string $url,
        string $username,
        string $password,
        string $tableName,
    ): self {
        return new self(
            url: $url,
            username: $username,
            password: $password,
            database: null,
            tableName: $tableName,
            setFields: ['url' => true, 'username' => true, 'password' => true, 'tableName' => true]
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
            'username' => $this->username,
            'password' => $this->password,
            'tableName' => $this->tableName];

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
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'ClickhouseConfig'),
            username: \Svix\Utils::deserializeString($data, 'username', true, 'ClickhouseConfig'),
            password: \Svix\Utils::deserializeString($data, 'password', true, 'ClickhouseConfig'),
            database: \Svix\Utils::deserializeString($data, 'database', false, 'ClickhouseConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', true, 'ClickhouseConfig')
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
