<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event. */
class MessageAttemptRecoveredEventData implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $appId      the Application's ID
     * @param string|null $appUid     the Application's UID
     * @param string      $endpointId the Endpoint's ID
     * @param string|null $msgEventId the Message's UID
     * @param string      $msgId      the Message's ID
     */
    private function __construct(
        public readonly string $appId,
        public readonly string $endpointId,
        public readonly MessageAttemptFailedData $lastAttempt,
        public readonly string $msgId,
        public readonly ?string $appUid = null,
        public readonly ?string $msgEventId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptRecoveredEventData with required fields.
     */
    public static function create(
        string $appId,
        string $endpointId,
        MessageAttemptFailedData $lastAttempt,
        string $msgId,
    ): self {
        return new self(
            appId: $appId,
            appUid: null,
            endpointId: $endpointId,
            lastAttempt: $lastAttempt,
            msgEventId: null,
            msgId: $msgId,
            setFields: ['appId' => true, 'endpointId' => true, 'lastAttempt' => true, 'msgId' => true]
        );
    }

    public function withAppUid(?string $appUid): self
    {
        $setFields = $this->setFields;
        $setFields['appUid'] = true;

        return new self(
            appId: $this->appId,
            appUid: $appUid,
            endpointId: $this->endpointId,
            lastAttempt: $this->lastAttempt,
            msgEventId: $this->msgEventId,
            msgId: $this->msgId,
            setFields: $setFields
        );
    }

    public function withMsgEventId(?string $msgEventId): self
    {
        $setFields = $this->setFields;
        $setFields['msgEventId'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            endpointId: $this->endpointId,
            lastAttempt: $this->lastAttempt,
            msgEventId: $msgEventId,
            msgId: $this->msgId,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'appId' => $this->appId,
            'endpointId' => $this->endpointId,
            'lastAttempt' => $this->lastAttempt,
            'msgId' => $this->msgId];

        if (isset($this->setFields['appUid'])) {
            $data['appUid'] = $this->appUid;
        }
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
            appId: \Svix\Utils::deserializeString($data, 'appId', true, 'MessageAttemptRecoveredEventData'),
            appUid: \Svix\Utils::deserializeString($data, 'appUid', false, 'MessageAttemptRecoveredEventData'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'MessageAttemptRecoveredEventData'),
            lastAttempt: \Svix\Utils::deserializeObject($data, 'lastAttempt', true, 'MessageAttemptRecoveredEventData', [MessageAttemptFailedData::class, 'fromMixed']),
            msgEventId: \Svix\Utils::deserializeString($data, 'msgEventId', false, 'MessageAttemptRecoveredEventData'),
            msgId: \Svix\Utils::deserializeString($data, 'msgId', true, 'MessageAttemptRecoveredEventData')
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
