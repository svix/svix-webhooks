<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ElasticSearchConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $indexName,
        public readonly string $url,
        public readonly bool $refresh,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ElasticSearchConfigOut with required fields.
     */
    public static function create(
        string $indexName,
        string $url,
        bool $refresh,
    ): self {
        return new self(
            indexName: $indexName,
            url: $url,
            refresh: $refresh,
            setFields: ['indexName' => true, 'url' => true, 'refresh' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'indexName' => $this->indexName,
            'url' => $this->url,
            'refresh' => $this->refresh];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            indexName: \Svix\Utils::deserializeString($data, 'indexName', true, 'ElasticSearchConfigOut'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'ElasticSearchConfigOut'),
            refresh: \Svix\Utils::deserializeBool($data, 'refresh', true, 'ElasticSearchConfigOut')
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
