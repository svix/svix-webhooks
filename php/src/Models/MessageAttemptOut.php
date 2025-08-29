<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class MessageAttemptOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string $endpointId         the Endpoint's ID
     * @param string $id                 the MessageAttempt's ID
     * @param string $msgId              the Message's ID
     * @param int    $responseDurationMs response duration in milliseconds
     */
    private function __construct(
        public readonly string $endpointId,
        public readonly string $id,
        public readonly string $msgId,
        public readonly string $response,
        public readonly int $responseDurationMs,
        public readonly int $responseStatusCode,
        public readonly MessageStatus $status,
        public readonly MessageStatusText $statusText,
        public readonly \DateTimeImmutable $timestamp,
        public readonly MessageAttemptTriggerType $triggerType,
        public readonly string $url,
        public readonly ?MessageOut $msg = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of MessageAttemptOut with required fields.
     */
    public static function create(
        string $endpointId,
        string $id,
        string $msgId,
        string $response,
        int $responseDurationMs,
        int $responseStatusCode,
        MessageStatus $status,
        MessageStatusText $statusText,
        \DateTimeImmutable $timestamp,
        MessageAttemptTriggerType $triggerType,
        string $url,
    ): self {
        return new self(
            endpointId: $endpointId,
            id: $id,
            msg: null,
            msgId: $msgId,
            response: $response,
            responseDurationMs: $responseDurationMs,
            responseStatusCode: $responseStatusCode,
            status: $status,
            statusText: $statusText,
            timestamp: $timestamp,
            triggerType: $triggerType,
            url: $url,
            setFields: ['endpointId' => true, 'id' => true, 'msgId' => true, 'response' => true, 'responseDurationMs' => true, 'responseStatusCode' => true, 'status' => true, 'statusText' => true, 'timestamp' => true, 'triggerType' => true, 'url' => true]
        );
    }

    public function withMsg(?MessageOut $msg): self
    {
        $setFields = $this->setFields;
        $setFields['msg'] = true;

        return new self(
            endpointId: $this->endpointId,
            id: $this->id,
            msg: $msg,
            msgId: $this->msgId,
            response: $this->response,
            responseDurationMs: $this->responseDurationMs,
            responseStatusCode: $this->responseStatusCode,
            status: $this->status,
            statusText: $this->statusText,
            timestamp: $this->timestamp,
            triggerType: $this->triggerType,
            url: $this->url,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = ['endpointId' => $this->endpointId,
            'id' => $this->id,
            'msgId' => $this->msgId,
            'response' => $this->response,
            'responseDurationMs' => $this->responseDurationMs,
            'responseStatusCode' => $this->responseStatusCode,
            'status' => $this->status,
            'statusText' => $this->statusText,
            'timestamp' => $this->timestamp->format('c'),
            'triggerType' => $this->triggerType,
            'url' => $this->url];

        if (isset($this->setFields['msg'])) {
            $data['msg'] = $this->msg;
        }

        return $data;
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            endpointId: \Svix\Utils::deserializeString($data, 'endpointId', true, 'MessageAttemptOut'),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'MessageAttemptOut'),
            msg: \Svix\Utils::deserializeObject($data, 'msg', false, 'MessageAttemptOut', [MessageOut::class, 'fromMixed']),
            msgId: \Svix\Utils::deserializeString($data, 'msgId', true, 'MessageAttemptOut'),
            response: \Svix\Utils::deserializeString($data, 'response', true, 'MessageAttemptOut'),
            responseDurationMs: \Svix\Utils::deserializeInt($data, 'responseDurationMs', true, 'MessageAttemptOut'),
            responseStatusCode: \Svix\Utils::deserializeInt($data, 'responseStatusCode', true, 'MessageAttemptOut'),
            status: \Svix\Utils::deserializeObject($data, 'status', true, 'MessageAttemptOut', [MessageStatus::class, 'fromMixed']),
            statusText: \Svix\Utils::deserializeObject($data, 'statusText', true, 'MessageAttemptOut', [MessageStatusText::class, 'fromMixed']),
            timestamp: \Svix\Utils::deserializeDt($data, 'timestamp', true, 'MessageAttemptOut'),
            triggerType: \Svix\Utils::deserializeObject($data, 'triggerType', true, 'MessageAttemptOut', [MessageAttemptTriggerType::class, 'fromMixed']),
            url: \Svix\Utils::getValFromJson($data, 'url', true, 'MessageAttemptOut')
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
