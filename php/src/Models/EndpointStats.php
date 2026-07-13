<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointStats implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly int $success,
        public readonly int $pending,
        public readonly int $sending,
        public readonly int $fail,
        public readonly int $canceled,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointStats with required fields.
     */
    public static function create(
        int $success,
        int $pending,
        int $sending,
        int $fail,
        int $canceled,
    ): self {
        return new self(
            success: $success,
            pending: $pending,
            sending: $sending,
            fail: $fail,
            canceled: $canceled,
            setFields: ['success' => true, 'pending' => true, 'sending' => true, 'fail' => true, 'canceled' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'success' => $this->success,
            'pending' => $this->pending,
            'sending' => $this->sending,
            'fail' => $this->fail,
            'canceled' => $this->canceled];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            success: \Svix\Utils::deserializeInt($data, 'success', true, 'EndpointStats'),
            pending: \Svix\Utils::deserializeInt($data, 'pending', true, 'EndpointStats'),
            sending: \Svix\Utils::deserializeInt($data, 'sending', true, 'EndpointStats'),
            fail: \Svix\Utils::deserializeInt($data, 'fail', true, 'EndpointStats'),
            canceled: \Svix\Utils::deserializeInt($data, 'canceled', true, 'EndpointStats')
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
