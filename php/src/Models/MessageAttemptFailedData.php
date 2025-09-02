<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageAttemptFailedData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $id the MessageAttempt's ID
     */
    private function __construct(
        public readonly string $id,
        public readonly int $responseStatusCode,
        public readonly \DateTimeImmutable $timestamp,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptFailedData with required fields.
     */
    public static function create(
        string $id,
        int $responseStatusCode,
        \DateTimeImmutable $timestamp,
    ): self {
        return new self(
            id: $id,
            responseStatusCode: $responseStatusCode,
            timestamp: $timestamp,
            setFields: ['id' => true, 'responseStatusCode' => true, 'timestamp' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'id' => $this->id,
            'responseStatusCode' => $this->responseStatusCode,
            'timestamp' => $this->timestamp->format('c')];

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            id: \Svix\Utils::deserializeString($data, 'id', true, 'MessageAttemptFailedData'),
            responseStatusCode: \Svix\Utils::deserializeInt($data, 'responseStatusCode', true, 'MessageAttemptFailedData'),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'MessageAttemptFailedData')
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
