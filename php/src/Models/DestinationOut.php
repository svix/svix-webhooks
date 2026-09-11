<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

class DestinationOut implements \JsonSerializable
{
    private array $setFields = [];

    /**
     * @param string                $id         the Destination's ID
     * @param string|null           $uid        the Destination's UID
     * @param list<string>|null     $eventTypes
     * @param list<string>|null     $channels
     * @param array<string, string> $metadata
     */
    private function __construct(
        public readonly DestinationOutConfig $config,
        public readonly string $id,
        public readonly DestinationStatus $status,
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
     * Create an instance of DestinationOut with required fields.
     */
    public static function create(
        DestinationOutConfig $config,
        string $id,
        DestinationStatus $status,
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
            config: DestinationOutConfig::fromTypeAndConfig(
                \Svix\Utils::deserializeString($data, 'type', true, 'DestinationOut'),
                \Svix\Utils::getValFromJson($data, 'config', false, 'DestinationOut') ?? []
            ),
            id: \Svix\Utils::deserializeString($data, 'id', true, 'DestinationOut'),

            uid: \Svix\Utils::deserializeString($data, 'uid', false, 'DestinationOut'),

            status: \Svix\Utils::deserializeObject($data, 'status', true, 'DestinationOut', [DestinationStatus::class, 'fromMixed']),

            currentIterator: \Svix\Utils::deserializeString($data, 'currentIterator', true, 'DestinationOut'),

            failureReason: \Svix\Utils::deserializeString($data, 'failureReason', false, 'DestinationOut'),

            createdAt: \Svix\Utils::deserializeDt($data, 'createdAt', true, 'DestinationOut'),

            updatedAt: \Svix\Utils::deserializeDt($data, 'updatedAt', true, 'DestinationOut'),

            batchSize: \Svix\Utils::deserializeInt($data, 'batchSize', true, 'DestinationOut'),

            maxWaitSecs: \Svix\Utils::deserializeInt($data, 'maxWaitSecs', true, 'DestinationOut'),

            eventTypes: \Svix\Utils::getValFromJson($data, 'eventTypes', false, 'DestinationOut'),

            channels: \Svix\Utils::getValFromJson($data, 'channels', false, 'DestinationOut'),

            nextRetryAt: \Svix\Utils::deserializeDt($data, 'nextRetryAt', false, 'DestinationOut'),

            metadata: \Svix\Utils::getValFromJson($data, 'metadata', true, 'DestinationOut')
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
