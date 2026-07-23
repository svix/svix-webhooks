<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SinkHttpConfig implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $headers
     */
    private function __construct(
        public readonly string $url,
        public readonly ?array $headers = null,
        public readonly ?string $key = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SinkHttpConfig with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            url: $url,
            headers: null,
            key: null,
            setFields: ['url' => true]
        );
    }

    public function withHeaders(?array $headers): self
    {
        $setFields = $this->setFields;
        $setFields['headers'] = true;

        return new self(
            url: $this->url,
            headers: $headers,
            key: $this->key,
            setFields: $setFields
        );
    }

    public function withKey(?string $key): self
    {
        $setFields = $this->setFields;
        $setFields['key'] = true;

        return new self(
            url: $this->url,
            headers: $this->headers,
            key: $key,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url];

        if (null !== $this->headers) {
            $data['headers'] = $this->headers;
        }
        if (isset($this->setFields['key'])) {
            $data['key'] = $this->key;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'SinkHttpConfig'),
            headers: \Svix\Utils::getValFromJson($data, 'headers', false, 'SinkHttpConfig'),
            key: \Svix\Utils::deserializeString($data, 'key', false, 'SinkHttpConfig')
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
