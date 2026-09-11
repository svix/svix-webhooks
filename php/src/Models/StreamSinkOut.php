<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class StreamSinkOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id         the sink's ID
     * @param string|null           $uid        the sink's UID
     * @param list<string>|null     $eventTypes
     * @param list<string>|null     $channels
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly StreamSinkOutConfig $config,
        public readonly string $id,
        public readonly SinkStatus $status,
        public readonly string $currentIterator,
        public readonly \DateTimeImmutable $createdAt,
        public readonly \DateTimeImmutable $updatedAt,
        public readonly int $batchSize,
        public readonly int $maxWaitSecs,
        public readonly array $metadata,
        public readonly ?string $uid = null,
        public readonly ?string $failureReason = null,
        public readonly ?array $eventTypes = null,
        public readonly ?array $channels = null,
        public readonly ?\DateTimeImmutable $nextRetryAt = null,
        array $setFields = [],
    ) {
        $this->setFields = $setFields;
    }

    /**
     * Create an instance of StreamSinkOut with required fields.
     */
    public static function create(
        StreamSinkOutConfig $config,
        string $id,
        SinkStatus $status,
        string $currentIterator,
        \DateTimeImmutable $createdAt,
        \DateTimeImmutable $updatedAt,
        int $batchSize,
        int $maxWaitSecs,
        array $metadata,
    ): self {
        return new self(
            config: $config,
            id: $id,
            uid: null,
            status: $status,
            currentIterator: $currentIterator,
            failureReason: null,
            createdAt: $createdAt,
            updatedAt: $updatedAt,
            batchSize: $batchSize,
            maxWaitSecs: $maxWaitSecs,
            eventTypes: null,
            channels: null,
            nextRetryAt: null,
            metadata: $metadata,
            setFields: [
                'config' => true, 'id' => true, 'status' => true, 'currentIterator' => true, 'createdAt' => true, 'updatedAt' => true, 'batchSize' => true, 'maxWaitSecs' => true, 'metadata' => true, ]
        );
    }

    public function withUid(?string $uid): self
    {
        $setFields = $this->setFields;
        $setFields['uid'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withFailureReason(?string $failureReason): self
    {
        $setFields = $this->setFields;
        $setFields['failureReason'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withEventTypes(?array $eventTypes): self
    {
        $setFields = $this->setFields;
        $setFields['eventTypes'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $eventTypes,
            channels: $this->channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withChannels(?array $channels): self
    {
        $setFields = $this->setFields;
        $setFields['channels'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $channels,
            nextRetryAt: $this->nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function withNextRetryAt(?\DateTimeImmutable $nextRetryAt): self
    {
        $setFields = $this->setFields;
        $setFields['nextRetryAt'] = true;

        return new self(
            config: $this->config,
            id: $this->id,
            uid: $this->uid,
            status: $this->status,
            currentIterator: $this->currentIterator,
            failureReason: $this->failureReason,
            createdAt: $this->createdAt,
            updatedAt: $this->updatedAt,
            batchSize: $this->batchSize,
            maxWaitSecs: $this->maxWaitSecs,
            eventTypes: $this->eventTypes,
            channels: $this->channels,
            nextRetryAt: $nextRetryAt,
            metadata: $this->metadata,
            setFields: $setFields
        );
    }

    public function jsonSerialize(): mixed
    {
        $data = [
            'type' => $this->config->variantName(),
            'config' => $this->config,
            'id' => $this->id,
            'status' => $this->status,
            'currentIterator' => $this->currentIterator,
            'createdAt' => $this->createdAt->format('c'),
            'updatedAt' => $this->updatedAt->format('c'),
            'batchSize' => $this->batchSize,
            'maxWaitSecs' => $this->maxWaitSecs,
            'metadata' => $this->metadata,
        ];

        if (isset($this->setFields['uid'])) {
            $data['uid'] = $this->uid;
        }
        if (isset($this->setFields['failureReason'])) {
            $data['failureReason'] = $this->failureReason;
        }
        if (null !== $this->eventTypes) {
            $data['eventTypes'] = $this->eventTypes;
        }
        if (null !== $this->channels) {
            $data['channels'] = $this->channels;
        }
        if (isset($this->setFields['nextRetryAt'])) {
            $data['nextRetryAt'] = $this->nextRetryAt->format('c');
        }

        return \Svix\Utils::newStdClassIfArrayIsEmpty($data);
    }

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return new self(
            config: StreamSinkOutConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'StreamSinkOut'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'StreamSinkOut') ?? []
            ),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'StreamSinkOut'),

            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'StreamSinkOut'),

            status: \Svix\Utils::deserializeObject($data, 'status', true, 'StreamSinkOut', [SinkStatus::class, 'fromMixed']),

            currentIterator: \Svix\Utils::deserializeString($data, 'currentIterator', true, 'StreamSinkOut'),

            failureReason: \Svix\Utils::deserializeString($data, 'failureReason', false, 'StreamSinkOut'),

            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'StreamSinkOut'),

            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'StreamSinkOut'),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', true, 'StreamSinkOut'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', true, 'StreamSinkOut'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'StreamSinkOut'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'StreamSinkOut'),

            nextRetryAt: \Svix\Utils::deserializeDt($data, 'nextRetryAt', false, 'StreamSinkOut'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'StreamSinkOut')
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
