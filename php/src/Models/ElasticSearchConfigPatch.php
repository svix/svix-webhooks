<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class ElasticSearchConfigPatch implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?string $indexName = null,
        public readonly ?string $url = null,
        public readonly ?string $apiKey = null,
        public readonly ?bool $refresh = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ElasticSearchConfigPatch with required fields.
     */
    public static function create(
    ): self {
        return new self(
            indexName: null,
            url: null,
            apiKey: null,
            refresh: null,
            setFields: []
        );
    }

    public function withIndexName(?string $indexName): self
    {
        $setFields = $this->setFields;
        $setFields['indexName'] = true;

        return new self(
            indexName: $indexName,
            url: $this->url,
            apiKey: $this->apiKey,
            refresh: $this->refresh,
            setFields: $setFields
        );
    }

    public function withUrl(?string $url): self
    {
        $setFields = $this->setFields;
        $setFields['url'] = true;

        return new self(
            indexName: $this->indexName,
            url: $url,
            apiKey: $this->apiKey,
            refresh: $this->refresh,
            setFields: $setFields
        );
    }

    public function withApiKey(?string $apiKey): self
    {
        $setFields = $this->setFields;
        $setFields['apiKey'] = true;

        return new self(
            indexName: $this->indexName,
            url: $this->url,
            apiKey: $apiKey,
            refresh: $this->refresh,
            setFields: $setFields
        );
    }

    public function withRefresh(?bool $refresh): self
    {
        $setFields = $this->setFields;
        $setFields['refresh'] = true;

        return new self(
            indexName: $this->indexName,
            url: $this->url,
            apiKey: $this->apiKey,
            refresh: $refresh,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (null !== $this->indexName) {
            $data['indexName'] = $this->indexName;
        }
        if (null !== $this->url) {
            $data['url'] = $this->url;
        }
        if (null !== $this->apiKey) {
            $data['apiKey'] = $this->apiKey;
        }
        if (null !== $this->refresh) {
            $data['refresh'] = $this->refresh;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            indexName: \Svix\Utils::deserializeString($data, 'indexName', false, 'ElasticSearchConfigPatch'),
            url: \Svix\Utils::getValFromJson($data, 'url', false, 'ElasticSearchConfigPatch'),
            apiKey: \Svix\Utils::deserializeString($data, 'apiKey', false, 'ElasticSearchConfigPatch'),
            refresh: \Svix\Utils::deserializeBool($data, 'refresh', false, 'ElasticSearchConfigPatch')
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
