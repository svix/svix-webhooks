<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PostgresConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $url,
        public readonly string $tableName,
        public readonly ?string $sslRootCert = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PostgresConfigOut with required fields.
     */
    public static function create(
        string $url,
        string $tableName,
    ): self {
        return new self(
            url: $url,
            tableName: $tableName,
            sslRootCert: null,
            setFields: ['url' => true, 'tableName' => true]
        );
    }

    public function withSslRootCert(?string $sslRootCert): self
    {
        $setFields = $this->setFields;
        $setFields['sslRootCert'] = true;

        return new self(
            url: $this->url,
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
            url: \Svix\Utils::deserializeString($data, 'url', true, 'PostgresConfigOut'),
            tableName: \Svix\Utils::deserializeString($data, 'tableName', true, 'PostgresConfigOut'),
            sslRootCert: \Svix\Utils::deserializeString($data, 'sslRootCert', false, 'PostgresConfigOut')
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
