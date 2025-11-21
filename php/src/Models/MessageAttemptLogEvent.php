<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent after message attempts are made. Contains metadata about message attempts and their results. In order to reduce the frequency of webhooks, these are sent in batches periodically. */
class MessageAttemptLogEvent implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param list<MessageAttemptLog> $data
     */
    private function __construct(
        public readonly array $data,
        public readonly string $type,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptLogEvent with required fields.
     */
    public static function create(
        array $data,
        string $type,
    ): self {
        return new self(
            data: $data,
            type: $type,
            setFields: ['data' => true, 'type' => true]
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'data' => $this->data,
            'type' => $this->type];

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            data: \Svix\Utils::deserializeObjectArray($data, 'data', true, 'MessageAttemptLogEvent', [MessageAttemptLog::class, 'fromMixed']),
            type: \Svix\Utils::getValFromJson($data, 'type', true, 'MessageAttemptLogEvent')
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
