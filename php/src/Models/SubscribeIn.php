<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SubscribeIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly ?EndpointIn $endpoint = null,
        public readonly ?AutoConfigSinkType $sink = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SubscribeIn with required fields.
     */
    public static function create(
    ): self {
        return new self(
            endpoint: null,
            sink: null,
            setFields: []
        );
    }

    public function withEndpoint(?EndpointIn $endpoint): self
    {
        $setFields = $this->setFields;
        $setFields['endpoint'] = true;

        return new self(
            endpoint: $endpoint,
            sink: $this->sink,
            setFields: $setFields
        );
    }

    public function withSink(?AutoConfigSinkType $sink): self
    {
        $setFields = $this->setFields;
        $setFields['sink'] = true;

        return new self(
            endpoint: $this->endpoint,
            sink: $sink,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
        ];

        if (isset($this->setFields['endpoint'])) {
            $data['endpoint'] = $this->endpoint;
        }
        if (isset($this->setFields['sink'])) {
            $data['sink'] = $this->sink;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            endpoint: \Svix\Utils::deserializeObject($data, 'endpoint', false, 'SubscribeIn', [EndpointIn::class, 'fromMixed']),
            sink: \Svix\Utils::deserializeObject($data, 'sink', false, 'SubscribeIn', [AutoConfigSinkType::class, 'fromMixed'])
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
