<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PostgresConfigPatch implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $url = null,
        public readonly ?string $password = null,
        public readonly ?string $tableName = null,
        public readonly ?string $sslRootCert = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PostgresConfigPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            url: null,
            password: null,
            tableName: null,
            sslRootCert: null,
            setFields: []
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            url: $url,
            password: $this->password,
            tableName: $this->tableName,
            sslRootCert: $this->sslRootCert,
            setFields: $setFields
        );
    }

    public function withPassword(?string $password): self
    {
        $setFields = $this->setFields;
        $setFields['password'] = true;

        return new self(
            url: $this->url,
            password: $password,
            tableName: $this->tableName,
            sslRootCert: $this->sslRootCert,
            setFields: $setFields
        );
    }

    public function withTableName(?string $tableName): self
    {
        $setFields = $this->setFields;
        $setFields['tableName'] = true;

        return new self(
            url: $this->url,
            password: $this->password,
            tableName: $tableName,
            sslRootCert: $this->sslRootCert,
            setFields: $setFields
        );
    }

    public function withSslRootCert(?string $sslRootCert): self
    {
        $setFields = $this->setFields;
        $setFields['sslRootCert'] = true;

        return new self(
            url: $this->url,
            password: $this->password,
            tableName: $this->tableName,
            sslRootCert: $sslRootCert,
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
        if (null !== $this->password) {
            $data['password'] = $this->password;
        }
        if (null !== $this->tableName) {
            $data['tableName'] = $this->tableName;
        }
        if (null !== $this->sslRootCert) {
            $data['sslRootCert'] = $this->sslRootCert;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::deserializeString($data, 'url', false, 'PostgresConfigPatch'),
            password: \Svix\Utils::deserializeString($data, 'password', false, 'PostgresConfigPatch'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', false, 'PostgresConfigPatch'),
            sslRootCert: \Svix\Utils::deserializeString($data, 'sslRootCert', false, 'PostgresConfigPatch')
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
