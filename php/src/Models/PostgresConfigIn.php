<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PostgresConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $url PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.
     *
     * Do NOT embed a password here; use the `password` field instead.
     * @param string|null $password  password for the connection
     * @param string      $tableName Table to insert into. May be schema-qualified (e.g. `public.events`).
     *
     * Quote characters are not supported. Each dot-separated segment is automatically double-quoted when the query is built, so `public.events` becomes `"public"."events"`.
     * @param string|null $sslRootCert PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.
     *
     * Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca` or `sslmode=verify-full`. Without it, only the built-in public roots are trusted.
     */
    private function __construct(
        public readonly string $url,
        public readonly string $tableName,
        public readonly ?string $password = null,
        public readonly ?string $sslRootCert = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PostgresConfigIn with required fields.
     */
    public static function create(
        string $url,
        string $tableName,
    ): self {
        return new self(
            url: $url,
            password: null,
            tableName: $tableName,
            sslRootCert: null,
            setFields: ['url' => true, 'tableName' => true]
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
            'url' => $this->url,
            'tableName' => $this->tableName];

        if (isset($this->setFields['password'])) {
            $data['password'] = $this->password;
        }
        if (isset($this->setFields['sslRootCert'])) {
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
            url: \Svix\Utils::deserializeString($data, 'url', true, 'PostgresConfigIn'),
            password: \Svix\Utils::deserializeString($data, 'password', false, 'PostgresConfigIn'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', true, 'PostgresConfigIn'),
            sslRootCert: \Svix\Utils::deserializeString($data, 'sslRootCert', false, 'PostgresConfigIn')
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
