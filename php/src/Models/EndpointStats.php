<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointStats implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly int $fail,
        public readonly int $pending,
        public readonly int $sending,
        public readonly int $success,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointStats with required fields.
     */
    public static function create(
        int $fail,
        int $pending,
        int $sending,
        int $success,
    ): self {
        return new self(
            fail: $fail,
            pending: $pending,
            sending: $sending,
            success: $success,
            setFields: ['fail' => true, 'pending' => true, 'sending' => true, 'success' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'fail' => $this->fail,
            'pending' => $this->pending,
            'sending' => $this->sending,
            'success' => $this->success];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            fail: \Svix\Utils::deserializeInt($data, 'fail', true, 'EndpointStats'),
            pending: \Svix\Utils::deserializeInt($data, 'pending', true, 'EndpointStats'),
            sending: \Svix\Utils::deserializeInt($data, 'sending', true, 'EndpointStats'),
            success: \Svix\Utils::deserializeInt($data, 'success', true, 'EndpointStats')
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
