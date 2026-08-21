<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SinkHttpConfigOut implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly string $url,
        public readonly EndpointHeadersOut $headers,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SinkHttpConfigOut with required fields.
     */
    public static function create(
        string $url,
        EndpointHeadersOut $headers,
    ): self {
        return new self(
            url: $url,
            headers: $headers,
            setFields: ['url' => true, 'headers' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url,
            'headers' => $this->headers];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'SinkHttpConfigOut'),
            headers: \Svix\Utils::deserializeObject($data, 'headers', true, 'SinkHttpConfigOut', [EndpointHeadersOut::class, 'fromMixed'])
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
