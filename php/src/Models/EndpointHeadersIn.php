<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointHeadersIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string> $headers
     */
    private function __construct(
        public readonly array $headers,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointHeadersIn with required fields.
     */
    public static function create(
        array $headers,
    ): self {
        return new self(
            headers: $headers,
            setFields: ['headers' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['headers' => $this->headers];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            headers: \Svix\Utils::getValFromJson($data, 'headers', true, 'EndpointHeadersIn')
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
