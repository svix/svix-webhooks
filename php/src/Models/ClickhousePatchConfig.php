<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClickhousePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $database = null,
        public readonly ?string $password = null,
        public readonly ?string $tableName = null,
        public readonly ?string $url = null,
        public readonly ?string $username = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ClickhousePatchConfig with required fields.
     */
    public static function create(
    ): self {
        return new self(
            database: null,
            password: null,
            tableName: null,
            url: null,
            username: null,
            setFields: []
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

    public function withPassword(?string $password): self
    {
        $setFields = $this->setFields;
        $setFields['password'] = true;

        return new self(
            database: $this->database,
            password: $password,
            tableName: $this->tableName,
            url: $this->url,
            username: $this->username,
            setFields: $setFields
        );
    }

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            database: $this->database,
            password: $this->password,
            tableName: $tableName,
            url: $this->url,
            username: $this->username,
            setFields: $setFields
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            database: $this->database,
            password: $this->password,
            tableName: $this->tableName,
            url: $url,
            username: $this->username,
            setFields: $setFields
        );
    }

    public function withUsername(?string $username): self
    {
        $setFields = $this->setFields;
        $setFields['username'] = true;

        return new self(
            database: $this->database,
            password: $this->password,
            tableName: $this->tableName,
            url: $this->url,
            username: $username,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->database) {
            $data['database'] = $this->database;
        }
        if (null !== $this->password) {
            $data['password'] = $this->password;
        }
        if (null !== $this->tableName) {
            $data['tableName'] = $this->tableName;
        }
        if (null !== $this->url) {
            $data['url'] = $this->url;
        }
        if (null !== $this->username) {
            $data['username'] = $this->username;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            database: \Svix\Utils::deserializeString($data, 'database', false, 'ClickhousePatchConfig'),
            password: \Svix\Utils::deserializeString($data, 'password', false, 'ClickhousePatchConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'ClickhousePatchConfig'),
            url: \Svix\Utils::getValFromJson($data, 'url', false, 'ClickhousePatchConfig'),
            username: \Svix\Utils::deserializeString($data, 'username', false, 'ClickhousePatchConfig')
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
