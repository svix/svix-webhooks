<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointHeadersPatchIn implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param array<string, string> $headers
     * @param list<string>|null     $deleteHeaders A list of headers be be removed
     */
    private function __construct(
        public readonly array $headers,
        public readonly ?array $deleteHeaders = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointHeadersPatchIn with required fields.
     */
    public static function create(
        array $headers,
    ): self {
        return new self(
            headers: $headers,
            deleteHeaders: null,
            setFields: ['headers' => true]
        );
    }

    public function withDeleteHeaders(?array $deleteHeaders): self
    {
        $setFields = $this->setFields;
        $setFields['deleteHeaders'] = true;

        return new self(
            headers: $this->headers,
            deleteHeaders: $deleteHeaders,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'headers' => $this->headers];

        if (null !== $this->deleteHeaders) {
            $data['deleteHeaders'] = $this->deleteHeaders;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            headers: \Svix\Utils::getValFromJson($data, 'headers', true, 'EndpointHeadersPatchIn'),
            deleteHeaders: \Svix\Utils::getValFromJson($data, 'deleteHeaders', false, 'EndpointHeadersPatchIn')
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
