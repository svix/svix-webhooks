<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ClickhousePatchConfig implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $url = null,
        public readonly ?string $username = null,
        public readonly ?string $password = null,
        public readonly ?string $database = null,
        public readonly ?string $tableName = null,
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
            url: null,
            username: null,
            password: null,
            database: null,
            tableName: null,
            setFields: []
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            url: $url,
            username: $this->username,
            password: $this->password,
            database: $this->database,
            tableName: $this->tableName,
            setFields: $setFields
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

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            url: $this->url,
            username: $this->username,
            password: $this->password,
            database: $this->database,
            tableName: $tableName,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->url) {
            $data['url'] = $this->url;
        }
        if (null !== $this->username) {
            $data['username'] = $this->username;
        }
        if (null !== $this->password) {
            $data['password'] = $this->password;
        }
        if (null !== $this->database) {
            $data['database'] = $this->database;
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
            url: \Svix\Utils::getValFromJson($data, 'url', false, 'ClickhousePatchConfig'),
            username: \Svix\Utils::deserializeString($data, 'username', false, 'ClickhousePatchConfig'),
            password: \Svix\Utils::deserializeString($data, 'password', false, 'ClickhousePatchConfig'),
            database: \Svix\Utils::deserializeString($data, 'database', false, 'ClickhousePatchConfig'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'ClickhousePatchConfig')
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
