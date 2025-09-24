<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed four times as a "ingest.message.attempt.failing" event, or after it's recovered as a "ingest.message.attempt.recovered" event. */
class IngestMessageAttemptRecoveredEventData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $endpointId the Endpoint's ID
     * @param string|null $msgEventId the Message's UID
     * @param string      $msgId      the Message's ID
     * @param string      $sourceId   the Source's ID
     */
    private function __construct(
        public readonly string $endpointId,
        public readonly MessageAttemptFailedData $lastAttempt,
        public readonly string $msgId,
        public readonly string $sourceId,
        public readonly ?string $msgEventId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of IngestMessageAttemptRecoveredEventData with required fields.
     */
    public static function create(
        string $endpointId,
        MessageAttemptFailedData $lastAttempt,
        string $msgId,
        string $sourceId,
    ): self {
        return new self(
            endpointId: $endpointId,
            lastAttempt: $lastAttempt,
            msgEventId: null,
            msgId: $msgId,
            sourceId: $sourceId,
            setFields: ['endpointId' => true, 'lastAttempt' => true, 'msgId' => true, 'sourceId' => true]
        );
    }

    public function withMsgEventId(?string $msgEventId): self
    {
        $setFields = $this->setFields;
        $setFields['msgEventId'] = true;

        return new self(
            endpointId: $this->endpointId,
            lastAttempt: $this->lastAttempt,
            msgEventId: $msgEventId,
            msgId: $this->msgId,
            sourceId: $this->sourceId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'endpointId' => $this->endpointId,
            'lastAttempt' => $this->lastAttempt,
            'msgId' => $this->msgId,
            'sourceId' => $this->sourceId];

        if (isset($this->setFields['msgEventId'])) {
            $data['msgEventId'] = $this->msgEventId;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'IngestMessageAttemptRecoveredEventData'),
            lastAttempt: \Svix\Utils::deserializeObject($data, 'lastAttempt', true, 'IngestMessageAttemptRecoveredEventData', [MessageAttemptFailedData::class, 'fromMixed']),
            msgEventId: \Svix\Utils::deserializeString($data, 'msgEventId', false, 'IngestMessageAttemptRecoveredEventData'),
            msgId: \Svix\Utils::deserializeString($data, 'msgId', true, 'IngestMessageAttemptRecoveredEventData'),
            sourceId: \Svix\Utils::deserializeString($data, 'sourceId', true, 'IngestMessageAttemptRecoveredEventData')
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
