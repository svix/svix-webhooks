<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageAttemptOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param int    $responseDurationMs response duration in milliseconds
     * @param string $msgId              the Message's ID
     * @param string $endpointId         the Endpoint's ID
     * @param string $id                 the MessageAttempt's ID
     */
    private function __construct(
        public readonly string $url,
        public readonly string $response,
        public readonly int $responseStatusCode,
        public readonly int $responseDurationMs,
        public readonly MessageStatus $status,
        public readonly MessageStatusText $statusText,
        public readonly MessageAttemptTriggerType $triggerType,
        public readonly string $msgId,
        public readonly string $endpointId,
        public readonly string $id,
        public readonly \DateTimeImmutable $timestamp,
        public readonly ?MessageOut $msg = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptOut with required fields.
     */
    public static function create(
        string $url,
        string $response,
        int $responseStatusCode,
        int $responseDurationMs,
        MessageStatus $status,
        MessageStatusText $statusText,
        MessageAttemptTriggerType $triggerType,
        string $msgId,
        string $endpointId,
        string $id,
        \DateTimeImmutable $timestamp,
    ): self {
        return new self(
            url: $url,
            response: $response,
            responseStatusCode: $responseStatusCode,
            responseDurationMs: $responseDurationMs,
            status: $status,
            statusText: $statusText,
            triggerType: $triggerType,
            msgId: $msgId,
            endpointId: $endpointId,
            id: $id,
            timestamp: $timestamp,
            msg: null,
            setFields: ['url' => true, 'response' => true, 'responseStatusCode' => true, 'responseDurationMs' => true, 'status' => true, 'statusText' => true, 'triggerType' => true, 'msgId' => true, 'endpointId' => true, 'id' => true, 'timestamp' => true]
        );
    }

    public function withMsg(?MessageOut $msg): self
    {
        $setFields = $this->setFields;
        $setFields['msg'] = true;

        return new self(
            url: $this->url,
            response: $this->response,
            responseStatusCode: $this->responseStatusCode,
            responseDurationMs: $this->responseDurationMs,
            status: $this->status,
            statusText: $this->statusText,
            triggerType: $this->triggerType,
            msgId: $this->msgId,
            endpointId: $this->endpointId,
            id: $this->id,
            timestamp: $this->timestamp,
            msg: $msg,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'url' => $this->url,
            'response' => $this->response,
            'responseStatusCode' => $this->responseStatusCode,
            'responseDurationMs' => $this->responseDurationMs,
            'status' => $this->status,
            'statusText' => $this->statusText,
            'triggerType' => $this->triggerType,
            'msgId' => $this->msgId,
            'endpointId' => $this->endpointId,
            'id' => $this->id,
            'timestamp' => $this->timestamp->format('c')];

        if (isset($this->setFields['msg'])) {
            $data['msg'] = $this->msg;
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'MessageAttemptOut'),
            response: \Svix\Utils::deserializeString($data, 'response', true, 'MessageAttemptOut'),
            responseStatusCode: \Svix\Utils::deserializeInt($data, 'responseStatusCode', true, 'MessageAttemptOut'),
            responseDurationMs: \Svix\Utils::deserializeInt($data, 'responseDurationMs', true, 'MessageAttemptOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'MessageAttemptOut', [MessageStatus::class, 'fromMixed']),
            statusText: \Svix\Utils::deserializeObject($data, 'statusText', true, 'MessageAttemptOut', [MessageStatusText::class, 'fromMixed']),
            triggerType: \Svix\Utils::deserializeObject($data, 'triggerType', true, 'MessageAttemptOut', [MessageAttemptTriggerType::class, 'fromMixed']),
            msgId: \Svix\Utils::deserializeString($data, 'msgId', true, 'MessageAttemptOut'),
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'MessageAttemptOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'MessageAttemptOut'),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'MessageAttemptOut'),
            msg: \Svix\Utils::deserializeObject($data, 'msg', false, 'MessageAttemptOut', [MessageOut::class, 'fromMixed'])
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
