<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class SubscribeIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly EndpointIn $endpoint,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of SubscribeIn with required fields.
     */
    public static function create(
        EndpointIn $endpoint,
    ): self {
        return new self(
            endpoint: $endpoint,
            setFields: ['endpoint' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'endpoint' => $this->endpoint];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            endpoint: \Svix\Utils::deserializeObject($data, 'endpoint', true, 'SubscribeIn', [EndpointIn::class, 'fromMixed'])
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
