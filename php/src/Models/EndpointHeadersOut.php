<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/**
 * The value of the headers is returned in the `headers` field.
 *
 * Sensitive headers that have been redacted are returned in the sensitive field.
 */
class EndpointHeadersOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string> $headers
     * @param list<string>          $sensitive
     */
    private function __construct(
        public readonly array $headers,
        public readonly array $sensitive,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointHeadersOut with required fields.
     */
    public static function create(
        array $headers,
        array $sensitive,
    ): self {
        return new self(
            headers: $headers,
            sensitive: $sensitive,
            setFields: ['headers' => true, 'sensitive' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'headers' => $this->headers,
            'sensitive' => $this->sensitive];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            headers: \Svix\Utils::getValFromJson($data, 'headers', true, 'EndpointHeadersOut'),
            sensitive: \Svix\Utils::getValFromJson($data, 'sensitive', true, 'EndpointHeadersOut')
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
