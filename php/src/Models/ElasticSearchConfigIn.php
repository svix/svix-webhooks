<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Configuration parameters for defining an ElasticSearch/OpenSearch sink. */
class ElasticSearchConfigIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $indexName Name of the index to write to
     *
     * This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch
     * @param string $url base URL to send indexing requests
     *
     * This should not include the /{index}/_bulk suffix
     * @param string|null $apiKey API key for authentication for ElasticSearch
     *
     * If not passed, any username:password embedded in the URL will be used. If none is passed,
     * the indexing will be done unauthenticated.
     * @param bool|null $refresh if true, Elasticsearch refreshes the affected shards to make this operation visible to search
     */
    private function __construct(
        public readonly string $indexName,
        public readonly string $url,
        public readonly ?string $apiKey = null,
        public readonly ?bool $refresh = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of ElasticSearchConfigIn with required fields.
     */
    public static function create(
        string $indexName,
        string $url,
    ): self {
        return new self(
            indexName: $indexName,
            url: $url,
            apiKey: null,
            refresh: null,
            setFields: ['indexName' => true, 'url' => true]
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
            'indexName' => $this->indexName,
            'url' => $this->url];

        if (isset($this->setFields['apiKey'])) {
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
            indexName: \Svix\Utils::deserializeString($data, 'indexName', true, 'ElasticSearchConfigIn'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'ElasticSearchConfigIn'),
            apiKey: \Svix\Utils::deserializeString($data, 'apiKey', false, 'ElasticSearchConfigIn'),
            refresh: \Svix\Utils::deserializeBool($data, 'refresh', false, 'ElasticSearchConfigIn')
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
