<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SinkOtelV1Config implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string>|null $headers
     */
    private function __construct(
        public readonly string $url,
        public readonly ?array $headers = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SinkOtelV1Config with required fields.
     */
    public static function create(
        string $url,
    ): self {
        return new self(
            headers: null,
            url: $url,
            setFields: ['url' => true]
        );
    }

    public function withHeaders(?array $headers): self
    {
        $setFields = $this->setFields;
        $setFields['headers'] = true;

        return new self(
            headers: $headers,
            url: $this->url,
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

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            headers: \Svix\Utils::getValFromJson($data, 'headers', false, 'SinkOtelV1Config'),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'SinkOtelV1Config')
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
