<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class HttpAttemptTimes implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly \DateTimeImmutable $end,
        public readonly \DateTimeImmutable $start,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of HttpAttemptTimes with required fields.
     */
    public static function create(
        \DateTimeImmutable $end,
        \DateTimeImmutable $start,
    ): self {
        return new self(
            end: $end,
            start: $start,
            setFields: ['end' => true, 'start' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'end' => $this->end->format('c'),
            'start' => $this->start->format('c')];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            end: \Svix\Utils::deserializeDt($data, 'end', true, 'HttpAttemptTimes'),
            start: \Svix\Utils::deserializeDt($data, 'start', true, 'HttpAttemptTimes')
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
