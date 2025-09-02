<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class PollingEndpointConsumerSeekIn implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly \DateTimeImmutable $after,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of PollingEndpointConsumerSeekIn with required fields.
     */
    public static function create(
        \DateTimeImmutable $after,
    ): self {
        return new self(
            after: $after,
            setFields: ['after' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'after' => $this->after->format('c')];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            after: \Svix\Utils::deserializeDt($data, 'after', true, 'PollingEndpointConsumerSeekIn')
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
