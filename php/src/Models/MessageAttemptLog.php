<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageAttemptLog implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string      $appId      the Application's ID
     * @param string|null $appUid     the Application's UID
     * @param string      $attemptId  the MessageAttempt's ID
     * @param string      $endpointId the Endpoint's ID
     * @param string|null $eventType  The event type's name
     * @param string|null $msgEventId the Message's UID
     * @param string      $msgId      the Message's ID
     * @param string      $orgId      the Environment's ID
     */
    private function __construct(
        public readonly string $appId,
        public readonly int $attemptCount,
        public readonly \DateTimeImmutable $attemptEnd,
        public readonly string $attemptId,
        public readonly \DateTimeImmutable $attemptStart,
        public readonly string $endpointId,
        public readonly \DateTimeImmutable $msgCreated,
        public readonly string $msgId,
        public readonly string $orgId,
        public readonly int $responseStatusCode,
        public readonly MessageStatus $status,
        public readonly ?string $appUid = null,
        public readonly ?string $eventType = null,
        public readonly ?HttpAttemptTimes $httpTimes = null,
        public readonly ?string $msgEventId = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptLog with required fields.
     */
    public static function create(
        string $appId,
        int $attemptCount,
        \DateTimeImmutable $attemptEnd,
        string $attemptId,
        \DateTimeImmutable $attemptStart,
        string $endpointId,
        \DateTimeImmutable $msgCreated,
        string $msgId,
        string $orgId,
        int $responseStatusCode,
        MessageStatus $status,
    ): self {
        return new self(
            appId: $appId,
            appUid: null,
            attemptCount: $attemptCount,
            attemptEnd: $attemptEnd,
            attemptId: $attemptId,
            attemptStart: $attemptStart,
            endpointId: $endpointId,
            eventType: null,
            httpTimes: null,
            msgCreated: $msgCreated,
            msgEventId: null,
            msgId: $msgId,
            orgId: $orgId,
            responseStatusCode: $responseStatusCode,
            status: $status,
            setFields: ['appId' => true, 'attemptCount' => true, 'attemptEnd' => true, 'attemptId' => true, 'attemptStart' => true, 'endpointId' => true, 'msgCreated' => true, 'msgId' => true, 'orgId' => true, 'responseStatusCode' => true, 'status' => true]
        );
    }

    public function withAppUid(?string $appUid): self
    {
        $setFields = $this->setFields;
        $setFields['appUid'] = true;

        return new self(
            appId: $this->appId,
            appUid: $appUid,
            attemptCount: $this->attemptCount,
            attemptEnd: $this->attemptEnd,
            attemptId: $this->attemptId,
            attemptStart: $this->attemptStart,
            endpointId: $this->endpointId,
            eventType: $this->eventType,
            httpTimes: $this->httpTimes,
            msgCreated: $this->msgCreated,
            msgEventId: $this->msgEventId,
            msgId: $this->msgId,
            orgId: $this->orgId,
            responseStatusCode: $this->responseStatusCode,
            status: $this->status,
            setFields: $setFields
        );
    }

    public function withEventType(?string $eventType): self
    {
        $setFields = $this->setFields;
        $setFields['eventType'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            attemptCount: $this->attemptCount,
            attemptEnd: $this->attemptEnd,
            attemptId: $this->attemptId,
            attemptStart: $this->attemptStart,
            endpointId: $this->endpointId,
            eventType: $eventType,
            httpTimes: $this->httpTimes,
            msgCreated: $this->msgCreated,
            msgEventId: $this->msgEventId,
            msgId: $this->msgId,
            orgId: $this->orgId,
            responseStatusCode: $this->responseStatusCode,
            status: $this->status,
            setFields: $setFields
        );
    }

    public function withHttpTimes(?HttpAttemptTimes $httpTimes): self
    {
        $setFields = $this->setFields;
        $setFields['httpTimes'] = true;

        return new self(
            appId: $this->appId,
            appUid: $this->appUid,
            attemptCount: $this->attemptCount,
            attemptEnd: $this->attemptEnd,
            attemptId: $this->attemptId,
            attemptStart: $this->attemptStart,
            endpointId: $this->endpointId,
            eventType: $this->eventType,
            httpTimes: $httpTimes,
            msgCreated: $this->msgCreated,
            msgEventId: $this->msgEventId,
            msgId: $this->msgId,
            orgId: $this->orgId,
            responseStatusCode: $this->responseStatusCode,
            status: $this->status,
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
            attemptCount: $this->attemptCount,
            attemptEnd: $this->attemptEnd,
            attemptId: $this->attemptId,
            attemptStart: $this->attemptStart,
            endpointId: $this->endpointId,
            eventType: $this->eventType,
            httpTimes: $this->httpTimes,
            msgCreated: $this->msgCreated,
            msgEventId: $msgEventId,
            msgId: $this->msgId,
            orgId: $this->orgId,
            responseStatusCode: $this->responseStatusCode,
            status: $this->status,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'appId' => $this->appId,
            'attemptCount' => $this->attemptCount,
            'attemptEnd' => $this->attemptEnd->format('c'),
            'attemptId' => $this->attemptId,
            'attemptStart' => $this->attemptStart->format('c'),
            'endpointId' => $this->endpointId,
            'msgCreated' => $this->msgCreated->format('c'),
            'msgId' => $this->msgId,
            'orgId' => $this->orgId,
            'responseStatusCode' => $this->responseStatusCode,
            'status' => $this->status];

        if (isset($this->setFields['appUid'])) {
            $data['appUid'] = $this->appUid;
        }
        if (isset($this->setFields['eventType'])) {
            $data['eventType'] = $this->eventType;
        }
        if (isset($this->setFields['httpTimes'])) {
            $data['httpTimes'] = $this->httpTimes;
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
            appId: \Svix\Utils::deserializeString($data, 'appId', true, 'MessageAttemptLog'),
            appUid: \Svix\Utils::deserializeString($data, 'appUid', false, 'MessageAttemptLog'),
            attemptCount: \Svix\Utils::deserializeInt($data, 'attemptCount', true, 'MessageAttemptLog'),
            attemptEnd: \Svix\Utils::deserializeDt($data, 'attemptEnd', true, 'MessageAttemptLog'),
            attemptId: \Svix\Utils::deserializeString($data, 'attemptId', true, 'MessageAttemptLog'),
            attemptStart: \Svix\Utils::deserializeDt($data, 'attemptStart', true, 'MessageAttemptLog'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'MessageAttemptLog'),
            eventType: \Svix\Utils::deserializeString($data, 'eventType', false, 'MessageAttemptLog'),
            httpTimes: \Svix\Utils::deserializeObject($data, 'httpTimes', false, 'MessageAttemptLog', [HttpAttemptTimes::class, 'fromMixed']),
            msgCreated: \Svix\Utils::deserializeDt($data, 'msgCreated', true, 'MessageAttemptLog'),
            msgEventId: \Svix\Utils::deserializeString($data, 'msgEventId', false, 'MessageAttemptLog'),
            msgId: \Svix\Utils::deserializeString($data, 'msgId', true, 'MessageAttemptLog'),
            orgId: \Svix\Utils::deserializeString($data, 'orgId', true, 'MessageAttemptLog'),
            responseStatusCode: \Svix\Utils::deserializeInt($data, 'responseStatusCode', true, 'MessageAttemptLog'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'MessageAttemptLog', [MessageStatus::class, 'fromMixed'])
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
