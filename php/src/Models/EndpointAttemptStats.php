<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class EndpointAttemptStats implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly int $success,
        public readonly int $fail,
        public readonly int $canceled,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of EndpointAttemptStats with required fields.
     */
    public static function create(
        int $success,
        int $fail,
        int $canceled,
    ): self {
        return new self(
            success: $success,
            fail: $fail,
            canceled: $canceled,
            setFields: ['success' => true, 'fail' => true, 'canceled' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'success' => $this->success,
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
            success: \Svix\Utils::deserializeInt($data, 'success', true, 'EndpointAttemptStats'),
            fail: \Svix\Utils::deserializeInt($data, 'fail', true, 'EndpointAttemptStats'),
            canceled: \Svix\Utils::deserializeInt($data, 'canceled', true, 'EndpointAttemptStats')
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
