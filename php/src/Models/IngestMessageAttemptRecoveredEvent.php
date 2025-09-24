<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent on a successful dispatch after an earlier failure op webhook has already been sent. */
class IngestMessageAttemptRecoveredEvent implements \JsonSerializable
{
    private array $setFields = [];

    private function __construct(
        public readonly IngestMessageAttemptRecoveredEventData $data,
        public readonly string $type,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestMessageAttemptRecoveredEvent with required fields.
     */
    public static function create(
        IngestMessageAttemptRecoveredEventData $data,
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
            data: \Svix\Utils::deserializeObject($data, 'data', true, 'IngestMessageAttemptRecoveredEvent', [IngestMessageAttemptRecoveredEventData::class, 'fromMixed']),
            type: \Svix\Utils::getValFromJson($data, 'type', true, 'IngestMessageAttemptRecoveredEvent')
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
